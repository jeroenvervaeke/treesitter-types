#[derive(Debug, Clone)]
pub enum BindingPattern<'tree> {
    SimpleBindingPattern(::std::boxed::Box<SimpleBindingPattern<'tree>>),
    AliasPattern(::std::boxed::Box<AliasPattern<'tree>>),
    ConsPattern(::std::boxed::Box<ConsPattern<'tree>>),
    ConstructorPattern(::std::boxed::Box<ConstructorPattern<'tree>>),
    LazyPattern(::std::boxed::Box<LazyPattern<'tree>>),
    OrPattern(::std::boxed::Box<OrPattern<'tree>>),
    RangePattern(::std::boxed::Box<RangePattern<'tree>>),
    TagPattern(::std::boxed::Box<TagPattern<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindingPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_pattern" => Ok(Self::AliasPattern(::std::boxed::Box::new(
                <AliasPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cons_pattern" => Ok(Self::ConsPattern(::std::boxed::Box::new(
                <ConsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_pattern" => Ok(Self::ConstructorPattern(::std::boxed::Box::new(
                <ConstructorPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_pattern" => Ok(Self::LazyPattern(::std::boxed::Box::new(
                <LazyPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "or_pattern" => Ok(Self::OrPattern(::std::boxed::Box::new(
                <OrPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "range_pattern" => Ok(Self::RangePattern(::std::boxed::Box::new(
                <RangePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_pattern" => Ok(Self::TagPattern(::std::boxed::Box::new(
                <TagPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SimpleBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimpleBindingPattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for BindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleBindingPattern(inner) => inner.span(),
            Self::AliasPattern(inner) => inner.span(),
            Self::ConsPattern(inner) => inner.span(),
            Self::ConstructorPattern(inner) => inner.span(),
            Self::LazyPattern(inner) => inner.span(),
            Self::OrPattern(inner) => inner.span(),
            Self::RangePattern(inner) => inner.span(),
            Self::TagPattern(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassExpression<'tree> {
    SimpleClassExpression(::std::boxed::Box<SimpleClassExpression<'tree>>),
    ClassApplication(::std::boxed::Box<ClassApplication<'tree>>),
    ClassFunction(::std::boxed::Box<ClassFunction<'tree>>),
    LetClassExpression(::std::boxed::Box<LetClassExpression<'tree>>),
    LetOpenClassExpression(::std::boxed::Box<LetOpenClassExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_application" => Ok(Self::ClassApplication(::std::boxed::Box::new(
                <ClassApplication as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_function" => Ok(Self::ClassFunction(::std::boxed::Box::new(
                <ClassFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_class_expression" => Ok(Self::LetClassExpression(::std::boxed::Box::new(
                <LetClassExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_open_class_expression" => {
                Ok(Self::LetOpenClassExpression(::std::boxed::Box::new(
                    <LetOpenClassExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
                )))
            }
            _other => {
                if let Ok(v) =
                    <SimpleClassExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimpleClassExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ClassExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleClassExpression(inner) => inner.span(),
            Self::ClassApplication(inner) => inner.span(),
            Self::ClassFunction(inner) => inner.span(),
            Self::LetClassExpression(inner) => inner.span(),
            Self::LetOpenClassExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassField<'tree> {
    ClassInitializer(::std::boxed::Box<ClassInitializer<'tree>>),
    InheritanceDefinition(::std::boxed::Box<InheritanceDefinition<'tree>>),
    InstanceVariableDefinition(::std::boxed::Box<InstanceVariableDefinition<'tree>>),
    ItemExtension(::std::boxed::Box<ItemExtension<'tree>>),
    MethodDefinition(::std::boxed::Box<MethodDefinition<'tree>>),
    QuotedItemExtension(::std::boxed::Box<QuotedItemExtension<'tree>>),
    TypeParameterConstraint(::std::boxed::Box<TypeParameterConstraint<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_initializer" => Ok(Self::ClassInitializer(::std::boxed::Box::new(
                <ClassInitializer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "inheritance_definition" => Ok(Self::InheritanceDefinition(::std::boxed::Box::new(
                <InheritanceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_variable_definition" => {
                Ok(Self::InstanceVariableDefinition(::std::boxed::Box::new(
                    <InstanceVariableDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "item_extension" => Ok(Self::ItemExtension(::std::boxed::Box::new(
                <ItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_definition" => Ok(Self::MethodDefinition(::std::boxed::Box::new(
                <MethodDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_item_extension" => Ok(Self::QuotedItemExtension(::std::boxed::Box::new(
                <QuotedItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ClassField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassInitializer(inner) => inner.span(),
            Self::InheritanceDefinition(inner) => inner.span(),
            Self::InstanceVariableDefinition(inner) => inner.span(),
            Self::ItemExtension(inner) => inner.span(),
            Self::MethodDefinition(inner) => inner.span(),
            Self::QuotedItemExtension(inner) => inner.span(),
            Self::TypeParameterConstraint(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassFieldSpecification<'tree> {
    InheritanceSpecification(::std::boxed::Box<InheritanceSpecification<'tree>>),
    InstanceVariableSpecification(::std::boxed::Box<InstanceVariableSpecification<'tree>>),
    ItemExtension(::std::boxed::Box<ItemExtension<'tree>>),
    MethodSpecification(::std::boxed::Box<MethodSpecification<'tree>>),
    QuotedItemExtension(::std::boxed::Box<QuotedItemExtension<'tree>>),
    TypeParameterConstraint(::std::boxed::Box<TypeParameterConstraint<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassFieldSpecification<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "inheritance_specification" => {
                Ok(Self::InheritanceSpecification(::std::boxed::Box::new(
                    <InheritanceSpecification as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "instance_variable_specification" => {
                Ok(Self::InstanceVariableSpecification(::std::boxed::Box::new(
                    <InstanceVariableSpecification as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "item_extension" => Ok(Self::ItemExtension(::std::boxed::Box::new(
                <ItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_specification" => Ok(Self::MethodSpecification(::std::boxed::Box::new(
                <MethodSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_item_extension" => Ok(Self::QuotedItemExtension(::std::boxed::Box::new(
                <QuotedItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ClassFieldSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::InheritanceSpecification(inner) => inner.span(),
            Self::InstanceVariableSpecification(inner) => inner.span(),
            Self::ItemExtension(inner) => inner.span(),
            Self::MethodSpecification(inner) => inner.span(),
            Self::QuotedItemExtension(inner) => inner.span(),
            Self::TypeParameterConstraint(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassType<'tree> {
    SimpleClassType(::std::boxed::Box<SimpleClassType<'tree>>),
    ClassFunctionType(::std::boxed::Box<ClassFunctionType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_function_type" => Ok(Self::ClassFunctionType(::std::boxed::Box::new(
                <ClassFunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SimpleClassType as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimpleClassType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ClassType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleClassType(inner) => inner.span(),
            Self::ClassFunctionType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Constant<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Character(::std::boxed::Box<Character<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    QuotedString(::std::boxed::Box<QuotedString<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Constant<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character" => Ok(Self::Character(::std::boxed::Box::new(
                <Character as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_string" => Ok(Self::QuotedString(::std::boxed::Box::new(
                <QuotedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Constant<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::Character(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::QuotedString(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EffectPatternType<'tree> {
    SimplePattern(::std::boxed::Box<SimplePattern<'tree>>),
    ConstructorPattern(::std::boxed::Box<ConstructorPattern<'tree>>),
    LazyPattern(::std::boxed::Box<LazyPattern<'tree>>),
    TagPattern(::std::boxed::Box<TagPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EffectPatternType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor_pattern" => Ok(Self::ConstructorPattern(::std::boxed::Box::new(
                <ConstructorPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_pattern" => Ok(Self::LazyPattern(::std::boxed::Box::new(
                <LazyPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_pattern" => Ok(Self::TagPattern(::std::boxed::Box::new(
                <TagPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <SimplePattern as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimplePattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for EffectPatternType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimplePattern(inner) => inner.span(),
            Self::ConstructorPattern(inner) => inner.span(),
            Self::LazyPattern(inner) => inner.span(),
            Self::TagPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    SimpleExpression(::std::boxed::Box<SimpleExpression<'tree>>),
    ApplicationExpression(::std::boxed::Box<ApplicationExpression<'tree>>),
    AssertExpression(::std::boxed::Box<AssertExpression<'tree>>),
    ConsExpression(::std::boxed::Box<ConsExpression<'tree>>),
    ForExpression(::std::boxed::Box<ForExpression<'tree>>),
    FunExpression(::std::boxed::Box<FunExpression<'tree>>),
    FunctionExpression(::std::boxed::Box<FunctionExpression<'tree>>),
    IfExpression(::std::boxed::Box<IfExpression<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    LazyExpression(::std::boxed::Box<LazyExpression<'tree>>),
    LetExceptionExpression(::std::boxed::Box<LetExceptionExpression<'tree>>),
    LetExpression(::std::boxed::Box<LetExpression<'tree>>),
    LetModuleExpression(::std::boxed::Box<LetModuleExpression<'tree>>),
    LetOpenExpression(::std::boxed::Box<LetOpenExpression<'tree>>),
    MatchExpression(::std::boxed::Box<MatchExpression<'tree>>),
    SetExpression(::std::boxed::Box<SetExpression<'tree>>),
    SignExpression(::std::boxed::Box<SignExpression<'tree>>),
    TryExpression(::std::boxed::Box<TryExpression<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    WhileExpression(::std::boxed::Box<WhileExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "application_expression" => Ok(Self::ApplicationExpression(::std::boxed::Box::new(
                <ApplicationExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "assert_expression" => Ok(Self::AssertExpression(::std::boxed::Box::new(
                <AssertExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cons_expression" => Ok(Self::ConsExpression(::std::boxed::Box::new(
                <ConsExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_expression" => Ok(Self::ForExpression(::std::boxed::Box::new(
                <ForExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fun_expression" => Ok(Self::FunExpression(::std::boxed::Box::new(
                <FunExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_expression" => Ok(Self::FunctionExpression(::std::boxed::Box::new(
                <FunctionExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_expression" => Ok(Self::IfExpression(::std::boxed::Box::new(
                <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_expression" => Ok(Self::LazyExpression(::std::boxed::Box::new(
                <LazyExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_exception_expression" => Ok(Self::LetExceptionExpression(::std::boxed::Box::new(
                <LetExceptionExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_expression" => Ok(Self::LetExpression(::std::boxed::Box::new(
                <LetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_module_expression" => Ok(Self::LetModuleExpression(::std::boxed::Box::new(
                <LetModuleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_open_expression" => Ok(Self::LetOpenExpression(::std::boxed::Box::new(
                <LetOpenExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_expression" => Ok(Self::MatchExpression(::std::boxed::Box::new(
                <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "set_expression" => Ok(Self::SetExpression(::std::boxed::Box::new(
                <SetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sign_expression" => Ok(Self::SignExpression(::std::boxed::Box::new(
                <SignExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_expression" => Ok(Self::TryExpression(::std::boxed::Box::new(
                <TryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_expression" => Ok(Self::WhileExpression(::std::boxed::Box::new(
                <WhileExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SimpleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimpleExpression(::std::boxed::Box::new(v)))
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
            Self::SimpleExpression(inner) => inner.span(),
            Self::ApplicationExpression(inner) => inner.span(),
            Self::AssertExpression(inner) => inner.span(),
            Self::ConsExpression(inner) => inner.span(),
            Self::ForExpression(inner) => inner.span(),
            Self::FunExpression(inner) => inner.span(),
            Self::FunctionExpression(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::LazyExpression(inner) => inner.span(),
            Self::LetExceptionExpression(inner) => inner.span(),
            Self::LetExpression(inner) => inner.span(),
            Self::LetModuleExpression(inner) => inner.span(),
            Self::LetOpenExpression(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::SetExpression(inner) => inner.span(),
            Self::SignExpression(inner) => inner.span(),
            Self::TryExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::WhileExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixOperator<'tree> {
    AddOperator(::std::boxed::Box<AddOperator<'tree>>),
    AndOperator(::std::boxed::Box<AndOperator<'tree>>),
    AssignOperator(::std::boxed::Box<AssignOperator<'tree>>),
    ConcatOperator(::std::boxed::Box<ConcatOperator<'tree>>),
    MultOperator(::std::boxed::Box<MultOperator<'tree>>),
    OrOperator(::std::boxed::Box<OrOperator<'tree>>),
    PowOperator(::std::boxed::Box<PowOperator<'tree>>),
    RelOperator(::std::boxed::Box<RelOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "add_operator" => Ok(Self::AddOperator(::std::boxed::Box::new(
                <AddOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "and_operator" => Ok(Self::AndOperator(::std::boxed::Box::new(
                <AndOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "assign_operator" => Ok(Self::AssignOperator(::std::boxed::Box::new(
                <AssignOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concat_operator" => Ok(Self::ConcatOperator(::std::boxed::Box::new(
                <ConcatOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "mult_operator" => Ok(Self::MultOperator(::std::boxed::Box::new(
                <MultOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "or_operator" => Ok(Self::OrOperator(::std::boxed::Box::new(
                <OrOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pow_operator" => Ok(Self::PowOperator(::std::boxed::Box::new(
                <PowOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "rel_operator" => Ok(Self::RelOperator(::std::boxed::Box::new(
                <RelOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AddOperator(inner) => inner.span(),
            Self::AndOperator(inner) => inner.span(),
            Self::AssignOperator(inner) => inner.span(),
            Self::ConcatOperator(inner) => inner.span(),
            Self::MultOperator(inner) => inner.span(),
            Self::OrOperator(inner) => inner.span(),
            Self::PowOperator(inner) => inner.span(),
            Self::RelOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleExpression<'tree> {
    SimpleModuleExpression(::std::boxed::Box<SimpleModuleExpression<'tree>>),
    Functor(::std::boxed::Box<Functor<'tree>>),
    ModuleApplication(::std::boxed::Box<ModuleApplication<'tree>>),
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
    Structure(::std::boxed::Box<Structure<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "functor" => Ok(Self::Functor(::std::boxed::Box::new(
                <Functor as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_application" => Ok(Self::ModuleApplication(::std::boxed::Box::new(
                <ModuleApplication as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structure" => Ok(Self::Structure(::std::boxed::Box::new(
                <Structure as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SimpleModuleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimpleModuleExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ModuleExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleModuleExpression(inner) => inner.span(),
            Self::Functor(inner) => inner.span(),
            Self::ModuleApplication(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
            Self::Structure(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleType<'tree> {
    Extension(::std::boxed::Box<Extension<'tree>>),
    FunctorType(::std::boxed::Box<FunctorType<'tree>>),
    ModuleTypeConstraint(::std::boxed::Box<ModuleTypeConstraint<'tree>>),
    ModuleTypeOf(::std::boxed::Box<ModuleTypeOf<'tree>>),
    ModuleTypePath(::std::boxed::Box<ModuleTypePath<'tree>>),
    ParenthesizedModuleType(::std::boxed::Box<ParenthesizedModuleType<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "functor_type" => Ok(Self::FunctorType(::std::boxed::Box::new(
                <FunctorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_type_constraint" => Ok(Self::ModuleTypeConstraint(::std::boxed::Box::new(
                <ModuleTypeConstraint as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_type_of" => Ok(Self::ModuleTypeOf(::std::boxed::Box::new(
                <ModuleTypeOf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_type_path" => Ok(Self::ModuleTypePath(::std::boxed::Box::new(
                <ModuleTypePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_module_type" => {
                Ok(Self::ParenthesizedModuleType(::std::boxed::Box::new(
                    <ParenthesizedModuleType as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "signature" => Ok(Self::Signature(::std::boxed::Box::new(
                <Signature as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Extension(inner) => inner.span(),
            Self::FunctorType(inner) => inner.span(),
            Self::ModuleTypeConstraint(inner) => inner.span(),
            Self::ModuleTypeOf(inner) => inner.span(),
            Self::ModuleTypePath(inner) => inner.span(),
            Self::ParenthesizedModuleType(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterType<'tree> {
    AbstractType(::std::boxed::Box<AbstractType<'tree>>),
    Parameter(::std::boxed::Box<Parameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_type" => Ok(Self::AbstractType(::std::boxed::Box::new(
                <AbstractType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter" => Ok(Self::Parameter(::std::boxed::Box::new(
                <Parameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractType(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Pattern<'tree> {
    EffectPattern(::std::boxed::Box<EffectPattern<'tree>>),
    AliasPattern(::std::boxed::Box<AliasPattern<'tree>>),
    ConsPattern(::std::boxed::Box<ConsPattern<'tree>>),
    ExceptionPattern(::std::boxed::Box<ExceptionPattern<'tree>>),
    OrPattern(::std::boxed::Box<OrPattern<'tree>>),
    RangePattern(::std::boxed::Box<RangePattern<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_pattern" => Ok(Self::AliasPattern(::std::boxed::Box::new(
                <AliasPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cons_pattern" => Ok(Self::ConsPattern(::std::boxed::Box::new(
                <ConsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "exception_pattern" => Ok(Self::ExceptionPattern(::std::boxed::Box::new(
                <ExceptionPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "or_pattern" => Ok(Self::OrPattern(::std::boxed::Box::new(
                <OrPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "range_pattern" => Ok(Self::RangePattern(::std::boxed::Box::new(
                <RangePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <EffectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::EffectPattern(::std::boxed::Box::new(v)))
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
            Self::EffectPattern(inner) => inner.span(),
            Self::AliasPattern(inner) => inner.span(),
            Self::ConsPattern(inner) => inner.span(),
            Self::ExceptionPattern(inner) => inner.span(),
            Self::OrPattern(inner) => inner.span(),
            Self::RangePattern(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PolymorphicTypeType<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    PolymorphicType(::std::boxed::Box<PolymorphicType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PolymorphicTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "polymorphic_type" => Ok(Self::PolymorphicType(::std::boxed::Box::new(
                <PolymorphicType as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PolymorphicTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::PolymorphicType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SequenceExpressionType<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SequenceExpressionType<'tree> {
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
impl ::treesitter_types::Spanned for SequenceExpressionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SignatureItem<'tree> {
    ClassDefinition(::std::boxed::Box<ClassDefinition<'tree>>),
    ClassTypeDefinition(::std::boxed::Box<ClassTypeDefinition<'tree>>),
    ExceptionDefinition(::std::boxed::Box<ExceptionDefinition<'tree>>),
    External(::std::boxed::Box<External<'tree>>),
    FloatingAttribute(::std::boxed::Box<FloatingAttribute<'tree>>),
    IncludeModuleType(::std::boxed::Box<IncludeModuleType<'tree>>),
    ItemExtension(::std::boxed::Box<ItemExtension<'tree>>),
    ModuleDefinition(::std::boxed::Box<ModuleDefinition<'tree>>),
    ModuleTypeDefinition(::std::boxed::Box<ModuleTypeDefinition<'tree>>),
    OpenModule(::std::boxed::Box<OpenModule<'tree>>),
    QuotedItemExtension(::std::boxed::Box<QuotedItemExtension<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    ValueSpecification(::std::boxed::Box<ValueSpecification<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SignatureItem<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_definition" => Ok(Self::ClassDefinition(::std::boxed::Box::new(
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_type_definition" => Ok(Self::ClassTypeDefinition(::std::boxed::Box::new(
                <ClassTypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "exception_definition" => Ok(Self::ExceptionDefinition(::std::boxed::Box::new(
                <ExceptionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "external" => Ok(Self::External(::std::boxed::Box::new(
                <External as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_attribute" => Ok(Self::FloatingAttribute(::std::boxed::Box::new(
                <FloatingAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "include_module_type" => Ok(Self::IncludeModuleType(::std::boxed::Box::new(
                <IncludeModuleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_extension" => Ok(Self::ItemExtension(::std::boxed::Box::new(
                <ItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_definition" => Ok(Self::ModuleDefinition(::std::boxed::Box::new(
                <ModuleDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_type_definition" => Ok(Self::ModuleTypeDefinition(::std::boxed::Box::new(
                <ModuleTypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "open_module" => Ok(Self::OpenModule(::std::boxed::Box::new(
                <OpenModule as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_item_extension" => Ok(Self::QuotedItemExtension(::std::boxed::Box::new(
                <QuotedItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_specification" => Ok(Self::ValueSpecification(::std::boxed::Box::new(
                <ValueSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SignatureItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDefinition(inner) => inner.span(),
            Self::ClassTypeDefinition(inner) => inner.span(),
            Self::ExceptionDefinition(inner) => inner.span(),
            Self::External(inner) => inner.span(),
            Self::FloatingAttribute(inner) => inner.span(),
            Self::IncludeModuleType(inner) => inner.span(),
            Self::ItemExtension(inner) => inner.span(),
            Self::ModuleDefinition(inner) => inner.span(),
            Self::ModuleTypeDefinition(inner) => inner.span(),
            Self::OpenModule(inner) => inner.span(),
            Self::QuotedItemExtension(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::ValueSpecification(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SignedConstant<'tree> {
    Constant(::std::boxed::Box<Constant<'tree>>),
    SignedNumber(::std::boxed::Box<SignedNumber<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SignedConstant<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signed_number" => Ok(Self::SignedNumber(::std::boxed::Box::new(
                <SignedNumber as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Constant as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Constant(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SignedConstant<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constant(inner) => inner.span(),
            Self::SignedNumber(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleBindingPattern<'tree> {
    SignedConstant(::std::boxed::Box<SignedConstant<'tree>>),
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    ConstructorPath(::std::boxed::Box<ConstructorPath<'tree>>),
    Extension(::std::boxed::Box<Extension<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    LocalOpenPattern(::std::boxed::Box<LocalOpenPattern<'tree>>),
    PackagePattern(::std::boxed::Box<PackagePattern<'tree>>),
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    ParenthesizedPattern(::std::boxed::Box<ParenthesizedPattern<'tree>>),
    PolymorphicVariantPattern(::std::boxed::Box<PolymorphicVariantPattern<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
    RecordPattern(::std::boxed::Box<RecordPattern<'tree>>),
    Tag(::std::boxed::Box<Tag<'tree>>),
    TypedPattern(::std::boxed::Box<TypedPattern<'tree>>),
    ValueName(::std::boxed::Box<ValueName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleBindingPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_path" => Ok(Self::ConstructorPath(::std::boxed::Box::new(
                <ConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "local_open_pattern" => Ok(Self::LocalOpenPattern(::std::boxed::Box::new(
                <LocalOpenPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "package_pattern" => Ok(Self::PackagePattern(::std::boxed::Box::new(
                <PackagePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_pattern" => Ok(Self::ParenthesizedPattern(::std::boxed::Box::new(
                <ParenthesizedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "polymorphic_variant_pattern" => {
                Ok(Self::PolymorphicVariantPattern(::std::boxed::Box::new(
                    <PolymorphicVariantPattern as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_pattern" => Ok(Self::RecordPattern(::std::boxed::Box::new(
                <RecordPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag" => Ok(Self::Tag(::std::boxed::Box::new(
                <Tag as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_pattern" => Ok(Self::TypedPattern(::std::boxed::Box::new(
                <TypedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_name" => Ok(Self::ValueName(::std::boxed::Box::new(
                <ValueName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SignedConstant as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SignedConstant(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SimpleBindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SignedConstant(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::ConstructorPath(inner) => inner.span(),
            Self::Extension(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::LocalOpenPattern(inner) => inner.span(),
            Self::PackagePattern(inner) => inner.span(),
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ParenthesizedPattern(inner) => inner.span(),
            Self::PolymorphicVariantPattern(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::RecordPattern(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
            Self::TypedPattern(inner) => inner.span(),
            Self::ValueName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleClassExpression<'tree> {
    ClassPath(::std::boxed::Box<ClassPath<'tree>>),
    Extension(::std::boxed::Box<Extension<'tree>>),
    InstantiatedClass(::std::boxed::Box<InstantiatedClass<'tree>>),
    ObjectExpression(::std::boxed::Box<ObjectExpression<'tree>>),
    ParenthesizedClassExpression(::std::boxed::Box<ParenthesizedClassExpression<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
    TypedClassExpression(::std::boxed::Box<TypedClassExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleClassExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_path" => Ok(Self::ClassPath(::std::boxed::Box::new(
                <ClassPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instantiated_class" => Ok(Self::InstantiatedClass(::std::boxed::Box::new(
                <InstantiatedClass as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_expression" => Ok(Self::ObjectExpression(::std::boxed::Box::new(
                <ObjectExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_class_expression" => {
                Ok(Self::ParenthesizedClassExpression(::std::boxed::Box::new(
                    <ParenthesizedClassExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_class_expression" => Ok(Self::TypedClassExpression(::std::boxed::Box::new(
                <TypedClassExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleClassExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassPath(inner) => inner.span(),
            Self::Extension(inner) => inner.span(),
            Self::InstantiatedClass(inner) => inner.span(),
            Self::ObjectExpression(inner) => inner.span(),
            Self::ParenthesizedClassExpression(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::TypedClassExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleClassType<'tree> {
    ClassBodyType(::std::boxed::Box<ClassBodyType<'tree>>),
    ClassTypePath(::std::boxed::Box<ClassTypePath<'tree>>),
    Extension(::std::boxed::Box<Extension<'tree>>),
    InstantiatedClassType(::std::boxed::Box<InstantiatedClassType<'tree>>),
    LetOpenClassType(::std::boxed::Box<LetOpenClassType<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleClassType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_body_type" => Ok(Self::ClassBodyType(::std::boxed::Box::new(
                <ClassBodyType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_type_path" => Ok(Self::ClassTypePath(::std::boxed::Box::new(
                <ClassTypePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instantiated_class_type" => Ok(Self::InstantiatedClassType(::std::boxed::Box::new(
                <InstantiatedClassType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_open_class_type" => Ok(Self::LetOpenClassType(::std::boxed::Box::new(
                <LetOpenClassType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleClassType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassBodyType(inner) => inner.span(),
            Self::ClassTypePath(inner) => inner.span(),
            Self::Extension(inner) => inner.span(),
            Self::InstantiatedClassType(inner) => inner.span(),
            Self::LetOpenClassType(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleExpression<'tree> {
    Constant(::std::boxed::Box<Constant<'tree>>),
    ArrayExpression(::std::boxed::Box<ArrayExpression<'tree>>),
    ArrayGetExpression(::std::boxed::Box<ArrayGetExpression<'tree>>),
    BigarrayGetExpression(::std::boxed::Box<BigarrayGetExpression<'tree>>),
    CoercionExpression(::std::boxed::Box<CoercionExpression<'tree>>),
    ConstructorPath(::std::boxed::Box<ConstructorPath<'tree>>),
    Extension(::std::boxed::Box<Extension<'tree>>),
    FieldGetExpression(::std::boxed::Box<FieldGetExpression<'tree>>),
    HashExpression(::std::boxed::Box<HashExpression<'tree>>),
    ListExpression(::std::boxed::Box<ListExpression<'tree>>),
    LocalOpenExpression(::std::boxed::Box<LocalOpenExpression<'tree>>),
    MethodInvocation(::std::boxed::Box<MethodInvocation<'tree>>),
    NewExpression(::std::boxed::Box<NewExpression<'tree>>),
    ObjectCopyExpression(::std::boxed::Box<ObjectCopyExpression<'tree>>),
    ObjectExpression(::std::boxed::Box<ObjectExpression<'tree>>),
    OcamlyaccValue(::std::boxed::Box<OcamlyaccValue<'tree>>),
    PackageExpression(::std::boxed::Box<PackageExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
    RecordExpression(::std::boxed::Box<RecordExpression<'tree>>),
    StringGetExpression(::std::boxed::Box<StringGetExpression<'tree>>),
    Tag(::std::boxed::Box<Tag<'tree>>),
    TypedExpression(::std::boxed::Box<TypedExpression<'tree>>),
    ValuePath(::std::boxed::Box<ValuePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_expression" => Ok(Self::ArrayExpression(::std::boxed::Box::new(
                <ArrayExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "array_get_expression" => Ok(Self::ArrayGetExpression(::std::boxed::Box::new(
                <ArrayGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bigarray_get_expression" => Ok(Self::BigarrayGetExpression(::std::boxed::Box::new(
                <BigarrayGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "coercion_expression" => Ok(Self::CoercionExpression(::std::boxed::Box::new(
                <CoercionExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_path" => Ok(Self::ConstructorPath(::std::boxed::Box::new(
                <ConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_get_expression" => Ok(Self::FieldGetExpression(::std::boxed::Box::new(
                <FieldGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "hash_expression" => Ok(Self::HashExpression(::std::boxed::Box::new(
                <HashExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_expression" => Ok(Self::ListExpression(::std::boxed::Box::new(
                <ListExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "local_open_expression" => Ok(Self::LocalOpenExpression(::std::boxed::Box::new(
                <LocalOpenExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_invocation" => Ok(Self::MethodInvocation(::std::boxed::Box::new(
                <MethodInvocation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "new_expression" => Ok(Self::NewExpression(::std::boxed::Box::new(
                <NewExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_copy_expression" => Ok(Self::ObjectCopyExpression(::std::boxed::Box::new(
                <ObjectCopyExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_expression" => Ok(Self::ObjectExpression(::std::boxed::Box::new(
                <ObjectExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ocamlyacc_value" => Ok(Self::OcamlyaccValue(::std::boxed::Box::new(
                <OcamlyaccValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "package_expression" => Ok(Self::PackageExpression(::std::boxed::Box::new(
                <PackageExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_expression" => Ok(Self::RecordExpression(::std::boxed::Box::new(
                <RecordExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_get_expression" => Ok(Self::StringGetExpression(::std::boxed::Box::new(
                <StringGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag" => Ok(Self::Tag(::std::boxed::Box::new(
                <Tag as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_expression" => Ok(Self::TypedExpression(::std::boxed::Box::new(
                <TypedExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_path" => Ok(Self::ValuePath(::std::boxed::Box::new(
                <ValuePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Constant as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Constant(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SimpleExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constant(inner) => inner.span(),
            Self::ArrayExpression(inner) => inner.span(),
            Self::ArrayGetExpression(inner) => inner.span(),
            Self::BigarrayGetExpression(inner) => inner.span(),
            Self::CoercionExpression(inner) => inner.span(),
            Self::ConstructorPath(inner) => inner.span(),
            Self::Extension(inner) => inner.span(),
            Self::FieldGetExpression(inner) => inner.span(),
            Self::HashExpression(inner) => inner.span(),
            Self::ListExpression(inner) => inner.span(),
            Self::LocalOpenExpression(inner) => inner.span(),
            Self::MethodInvocation(inner) => inner.span(),
            Self::NewExpression(inner) => inner.span(),
            Self::ObjectCopyExpression(inner) => inner.span(),
            Self::ObjectExpression(inner) => inner.span(),
            Self::OcamlyaccValue(inner) => inner.span(),
            Self::PackageExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::RecordExpression(inner) => inner.span(),
            Self::StringGetExpression(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
            Self::TypedExpression(inner) => inner.span(),
            Self::ValuePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleModuleExpression<'tree> {
    Extension(::std::boxed::Box<Extension<'tree>>),
    PackedModule(::std::boxed::Box<PackedModule<'tree>>),
    ParenthesizedModuleExpression(::std::boxed::Box<ParenthesizedModuleExpression<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
    TypedModuleExpression(::std::boxed::Box<TypedModuleExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleModuleExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "packed_module" => Ok(Self::PackedModule(::std::boxed::Box::new(
                <PackedModule as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_module_expression" => {
                Ok(Self::ParenthesizedModuleExpression(::std::boxed::Box::new(
                    <ParenthesizedModuleExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_module_expression" => Ok(Self::TypedModuleExpression(::std::boxed::Box::new(
                <TypedModuleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleModuleExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Extension(inner) => inner.span(),
            Self::PackedModule(inner) => inner.span(),
            Self::ParenthesizedModuleExpression(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::TypedModuleExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimplePattern<'tree> {
    SignedConstant(::std::boxed::Box<SignedConstant<'tree>>),
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    ConstructorPath(::std::boxed::Box<ConstructorPath<'tree>>),
    Extension(::std::boxed::Box<Extension<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    LocalOpenPattern(::std::boxed::Box<LocalOpenPattern<'tree>>),
    PackagePattern(::std::boxed::Box<PackagePattern<'tree>>),
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    ParenthesizedPattern(::std::boxed::Box<ParenthesizedPattern<'tree>>),
    PolymorphicVariantPattern(::std::boxed::Box<PolymorphicVariantPattern<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
    RecordPattern(::std::boxed::Box<RecordPattern<'tree>>),
    Tag(::std::boxed::Box<Tag<'tree>>),
    TypedPattern(::std::boxed::Box<TypedPattern<'tree>>),
    ValuePattern(::std::boxed::Box<ValuePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimplePattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_path" => Ok(Self::ConstructorPath(::std::boxed::Box::new(
                <ConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "local_open_pattern" => Ok(Self::LocalOpenPattern(::std::boxed::Box::new(
                <LocalOpenPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "package_pattern" => Ok(Self::PackagePattern(::std::boxed::Box::new(
                <PackagePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_pattern" => Ok(Self::ParenthesizedPattern(::std::boxed::Box::new(
                <ParenthesizedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "polymorphic_variant_pattern" => {
                Ok(Self::PolymorphicVariantPattern(::std::boxed::Box::new(
                    <PolymorphicVariantPattern as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_pattern" => Ok(Self::RecordPattern(::std::boxed::Box::new(
                <RecordPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag" => Ok(Self::Tag(::std::boxed::Box::new(
                <Tag as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_pattern" => Ok(Self::TypedPattern(::std::boxed::Box::new(
                <TypedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_pattern" => Ok(Self::ValuePattern(::std::boxed::Box::new(
                <ValuePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SignedConstant as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SignedConstant(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SimplePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SignedConstant(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::ConstructorPath(inner) => inner.span(),
            Self::Extension(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::LocalOpenPattern(inner) => inner.span(),
            Self::PackagePattern(inner) => inner.span(),
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ParenthesizedPattern(inner) => inner.span(),
            Self::PolymorphicVariantPattern(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::RecordPattern(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
            Self::TypedPattern(inner) => inner.span(),
            Self::ValuePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleType<'tree> {
    ConstructedType(::std::boxed::Box<ConstructedType<'tree>>),
    Extension(::std::boxed::Box<Extension<'tree>>),
    HashType(::std::boxed::Box<HashType<'tree>>),
    LocalOpenType(::std::boxed::Box<LocalOpenType<'tree>>),
    ObjectType(::std::boxed::Box<ObjectType<'tree>>),
    PackageType(::std::boxed::Box<PackageType<'tree>>),
    ParenthesizedType(::std::boxed::Box<ParenthesizedType<'tree>>),
    PolymorphicVariantType(::std::boxed::Box<PolymorphicVariantType<'tree>>),
    QuotedExtension(::std::boxed::Box<QuotedExtension<'tree>>),
    TypeConstructorPath(::std::boxed::Box<TypeConstructorPath<'tree>>),
    TypeVariable(::std::boxed::Box<TypeVariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructed_type" => Ok(Self::ConstructedType(::std::boxed::Box::new(
                <ConstructedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension" => Ok(Self::Extension(::std::boxed::Box::new(
                <Extension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "hash_type" => Ok(Self::HashType(::std::boxed::Box::new(
                <HashType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "local_open_type" => Ok(Self::LocalOpenType(::std::boxed::Box::new(
                <LocalOpenType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_type" => Ok(Self::ObjectType(::std::boxed::Box::new(
                <ObjectType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "package_type" => Ok(Self::PackageType(::std::boxed::Box::new(
                <PackageType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_type" => Ok(Self::ParenthesizedType(::std::boxed::Box::new(
                <ParenthesizedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "polymorphic_variant_type" => Ok(Self::PolymorphicVariantType(::std::boxed::Box::new(
                <PolymorphicVariantType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_extension" => Ok(Self::QuotedExtension(::std::boxed::Box::new(
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_constructor_path" => Ok(Self::TypeConstructorPath(::std::boxed::Box::new(
                <TypeConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_variable" => Ok(Self::TypeVariable(::std::boxed::Box::new(
                <TypeVariable as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstructedType(inner) => inner.span(),
            Self::Extension(inner) => inner.span(),
            Self::HashType(inner) => inner.span(),
            Self::LocalOpenType(inner) => inner.span(),
            Self::ObjectType(inner) => inner.span(),
            Self::PackageType(inner) => inner.span(),
            Self::ParenthesizedType(inner) => inner.span(),
            Self::PolymorphicVariantType(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::TypeConstructorPath(inner) => inner.span(),
            Self::TypeVariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StructureItem<'tree> {
    ClassDefinition(::std::boxed::Box<ClassDefinition<'tree>>),
    ClassTypeDefinition(::std::boxed::Box<ClassTypeDefinition<'tree>>),
    ExceptionDefinition(::std::boxed::Box<ExceptionDefinition<'tree>>),
    External(::std::boxed::Box<External<'tree>>),
    FloatingAttribute(::std::boxed::Box<FloatingAttribute<'tree>>),
    IncludeModule(::std::boxed::Box<IncludeModule<'tree>>),
    ItemExtension(::std::boxed::Box<ItemExtension<'tree>>),
    ModuleDefinition(::std::boxed::Box<ModuleDefinition<'tree>>),
    ModuleTypeDefinition(::std::boxed::Box<ModuleTypeDefinition<'tree>>),
    OpenModule(::std::boxed::Box<OpenModule<'tree>>),
    QuotedItemExtension(::std::boxed::Box<QuotedItemExtension<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    ValueDefinition(::std::boxed::Box<ValueDefinition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructureItem<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_definition" => Ok(Self::ClassDefinition(::std::boxed::Box::new(
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_type_definition" => Ok(Self::ClassTypeDefinition(::std::boxed::Box::new(
                <ClassTypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "exception_definition" => Ok(Self::ExceptionDefinition(::std::boxed::Box::new(
                <ExceptionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "external" => Ok(Self::External(::std::boxed::Box::new(
                <External as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_attribute" => Ok(Self::FloatingAttribute(::std::boxed::Box::new(
                <FloatingAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "include_module" => Ok(Self::IncludeModule(::std::boxed::Box::new(
                <IncludeModule as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_extension" => Ok(Self::ItemExtension(::std::boxed::Box::new(
                <ItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_definition" => Ok(Self::ModuleDefinition(::std::boxed::Box::new(
                <ModuleDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_type_definition" => Ok(Self::ModuleTypeDefinition(::std::boxed::Box::new(
                <ModuleTypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "open_module" => Ok(Self::OpenModule(::std::boxed::Box::new(
                <OpenModule as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_item_extension" => Ok(Self::QuotedItemExtension(::std::boxed::Box::new(
                <QuotedItemExtension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_definition" => Ok(Self::ValueDefinition(::std::boxed::Box::new(
                <ValueDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructureItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDefinition(inner) => inner.span(),
            Self::ClassTypeDefinition(inner) => inner.span(),
            Self::ExceptionDefinition(inner) => inner.span(),
            Self::External(inner) => inner.span(),
            Self::FloatingAttribute(inner) => inner.span(),
            Self::IncludeModule(inner) => inner.span(),
            Self::ItemExtension(inner) => inner.span(),
            Self::ModuleDefinition(inner) => inner.span(),
            Self::ModuleTypeDefinition(inner) => inner.span(),
            Self::OpenModule(inner) => inner.span(),
            Self::QuotedItemExtension(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::ValueDefinition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Type<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    AliasedType(::std::boxed::Box<AliasedType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "aliased_type" => Ok(Self::AliasedType(::std::boxed::Box::new(
                <AliasedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Type<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::AliasedType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AbstractType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeConstructor<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TypeConstructor as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AbstractType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AddOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AddOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "add_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AddOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AddOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AliasPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: AliasPatternAlias<'tree>,
    pub pattern: AliasPatternPattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alias_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                <AliasPatternAlias as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <AliasPatternPattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AliasPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AliasedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: TypeVariable<'tree>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "aliased_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                <TypeVariable as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for AliasedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ApplicationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::std::vec::Vec<ApplicationExpressionArgument<'tree>>,
    pub function: SimpleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ApplicationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "application_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("argument", &mut cursor) {
                    items.push(
                        <ApplicationExpressionArgument as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ApplicationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrayBindingPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BindingPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayBindingPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_binding_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<BindingPattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayBindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrayExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for ArrayExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrayGetExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub array: SimpleExpression<'tree>,
    pub index: SequenceExpression<'tree>,
    pub operator: ::core::option::Option<IndexingOperatorPath<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayGetExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_get_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            array: {
                let child = node
                    .child_by_field_name("array")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("array", node))?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("index", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => Some(
                    <IndexingOperatorPath as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayGetExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrayPattern<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ArrayPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct AssertExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SimpleExpression<'tree>,
    pub children: ::core::option::Option<AttributeId<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssertExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assert_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    Some(&child) => Some(<AttributeId as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AssertExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AttributeChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
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
pub struct AttributeId<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeId<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_id");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AttributeId<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AttributeId<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributePayload<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributePayloadChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributePayload<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_payload");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AttributePayloadChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributePayload<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BigarrayGetExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub array: SimpleExpression<'tree>,
    pub index: SequenceExpression<'tree>,
    pub operator: ::core::option::Option<IndexingOperatorPath<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BigarrayGetExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bigarray_get_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            array: {
                let child = node
                    .child_by_field_name("array")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("array", node))?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("index", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => Some(
                    <IndexingOperatorPath as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for BigarrayGetExpression<'_> {
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
pub struct Character<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: CharacterContent<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Character<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <CharacterContent as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Character<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CharacterContent<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<EscapeSequence<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterContent<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <EscapeSequence as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CharacterContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassApplication<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::std::vec::Vec<ClassApplicationArgument<'tree>>,
    pub class: SimpleClassExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassApplication<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_application");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("argument", &mut cursor) {
                    items.push(
                        <ClassApplicationArgument as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            class: {
                let child = node
                    .child_by_field_name("class")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("class", node))?;
                <SimpleClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassApplication<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassBinding<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<ClassExpression<'tree>>,
    pub class_type: ::core::option::Option<ClassType<'tree>>,
    pub children: ::std::vec::Vec<ClassBindingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassBinding<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_binding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<ClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            class_type: match node.child_by_field_name("class_type") {
                Some(child) => Some(<ClassType as ::treesitter_types::FromNode>::from_node(
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
                        <ClassBindingChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassBinding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassBodyType<'tree> {
    pub span: ::treesitter_types::Span,
    pub self_type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<ClassBodyTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassBodyType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_body_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            self_type: match node.child_by_field_name("self_type") {
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
                        <ClassBodyTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassBodyType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassDefinition<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct ClassFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ClassExpression<'tree>,
    pub children: ::std::vec::Vec<Parameter<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <ClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(<Parameter as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassFunctionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub codomain: ClassType<'tree>,
    pub domain: ClassFunctionTypeDomain<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassFunctionType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_function_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            codomain: {
                let child = node.child_by_field_name("codomain").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("codomain", node)
                })?;
                <ClassType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            domain: {
                let child = node
                    .child_by_field_name("domain")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("domain", node))?;
                <ClassFunctionTypeDomain as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassFunctionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub initializer: SequenceExpression<'tree>,
    pub children: ::std::vec::Vec<ItemAttribute<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            initializer: {
                let child = node.child_by_field_name("initializer").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("initializer", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(<ItemAttribute as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassPath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassPathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassPath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassPathChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassTypeBinding<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SimpleClassType<'tree>,
    pub children: ::std::vec::Vec<ClassTypeBindingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassTypeBinding<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_type_binding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <SimpleClassType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ClassTypeBindingChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassTypeBinding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassTypeDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassTypeDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassTypeDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_type_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassTypeDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassTypeDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassTypePath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassTypePathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassTypePath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_type_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassTypePathChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassTypePath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CoercionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub coercion: Type<'tree>,
    pub expression: SequenceExpression<'tree>,
    pub r#type: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CoercionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "coercion_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            coercion: {
                let child = node.child_by_field_name("coercion").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("coercion", node)
                })?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for CoercionExpression<'_> {
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
pub struct ConsExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConsExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "cons_expression");
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConsExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConsPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ConsPatternLeft<'tree>,
    pub right: ConsPatternRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConsPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "cons_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <ConsPatternLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <ConsPatternRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConsPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstrainModule<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: ExtendedModulePath<'tree>,
    pub children: ModulePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstrainModule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constrain_module");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                <ExtendedModulePath as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <ModulePath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstrainModule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstrainModuleType<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: ModuleType<'tree>,
    pub children: ModuleTypePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstrainModuleType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constrain_module_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                <ModuleType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <ModuleTypePath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstrainModuleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstrainType<'tree> {
    pub span: ::treesitter_types::Span,
    pub equation: Type<'tree>,
    pub children: ::std::vec::Vec<ConstrainTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstrainType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constrain_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            equation: {
                let child = node.child_by_field_name("equation").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("equation", node)
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
                        <ConstrainTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstrainType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstructedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructed_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for ConstructedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstructorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct ConstructorPath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConstructorPathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorPath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ConstructorPathChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstructorPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstructorPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ConstructorPatternPattern<'tree>,
    pub children: ::std::vec::Vec<ConstructorPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <ConstructorPatternPattern as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ConstructorPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstructorPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Directive<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Directive<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Directive<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Directive<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DoClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<SequenceExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <SequenceExpression as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for DoClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EffectPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub continuation: SimplePattern<'tree>,
    pub effect: ::std::boxed::Box<EffectPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EffectPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "effect_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            continuation: {
                let child = node.child_by_field_name("continuation").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("continuation", node)
                })?;
                <SimplePattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            effect: {
                let child = node
                    .child_by_field_name("effect")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("effect", node))?;
                ::std::boxed::Box::new(<EffectPattern as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?)
            },
        })
    }
}
impl ::treesitter_types::Spanned for EffectPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ElseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: Expression<'tree>,
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
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ExceptionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExceptionDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExceptionDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exception_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ExceptionDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExceptionDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExceptionPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: Pattern<'tree>,
    pub children: ::core::option::Option<AttributeId<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExceptionPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exception_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                    Some(&child) => Some(<AttributeId as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExceptionPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExpressionItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expression_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ExpressionItemChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExpressionItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExtendedModulePath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExtendedModulePathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtendedModulePath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extended_module_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ExtendedModulePathChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExtendedModulePath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Extension<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExtensionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Extension<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extension");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ExtensionChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Extension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct External<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: PolymorphicType<'tree>,
    pub children: ::std::vec::Vec<ExternalChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for External<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "external");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <PolymorphicType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ExternalChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for External<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: PolymorphicType<'tree>,
    pub children: FieldName<'tree>,
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
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <PolymorphicType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <FieldName as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct FieldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Expression<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: FieldPath<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
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
                <FieldPath as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct FieldGetExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: FieldPath<'tree>,
    pub record: SimpleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldGetExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_get_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                <FieldPath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            record: {
                let child = node
                    .child_by_field_name("record")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("record", node))?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldGetExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldPath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldPathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <FieldPathChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ::core::option::Option<FieldPatternPattern<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: FieldPath<'tree>,
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
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(
                    <FieldPatternPattern as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                <FieldPath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FloatingAttribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FloatingAttributeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FloatingAttribute<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "floating_attribute");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <FloatingAttributeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FloatingAttribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub from: SequenceExpression<'tree>,
    pub name: ForExpressionName<'tree>,
    pub to: SequenceExpression<'tree>,
    pub children: ::std::vec::Vec<ForExpressionChildren<'tree>>,
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
            from: {
                let child = node
                    .child_by_field_name("from")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("from", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ForExpressionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            to: {
                let child = node
                    .child_by_field_name("to")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("to", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ForExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
pub struct FunExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SequenceExpression<'tree>,
    pub r#type: ::core::option::Option<SimpleType<'tree>>,
    pub children: ::std::vec::Vec<FunExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fun_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<SimpleType as ::treesitter_types::FromNode>::from_node(
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
                        <FunExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FunctionExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <FunctionExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub codomain: Type<'tree>,
    pub domain: FunctionTypeDomain<'tree>,
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
            codomain: {
                let child = node.child_by_field_name("codomain").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("codomain", node)
                })?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            domain: {
                let child = node
                    .child_by_field_name("domain")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("domain", node))?;
                <FunctionTypeDomain as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct Functor<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ModuleExpression<'tree>,
    pub children: ::std::vec::Vec<ModuleParameter<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Functor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "functor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ModuleParameter as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Functor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctorType<'tree> {
    pub span: ::treesitter_types::Span,
    pub codomain: ModuleType<'tree>,
    pub domain: ::core::option::Option<ModuleType<'tree>>,
    pub children: ::std::vec::Vec<ModuleParameter<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctorType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "functor_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            codomain: {
                let child = node.child_by_field_name("codomain").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("codomain", node)
                })?;
                <ModuleType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            domain: match node.child_by_field_name("domain") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
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
                        <ModuleParameter as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctorType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Guard<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SequenceExpression<'tree>,
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
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct HashExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: SimpleExpression<'tree>,
    pub operator: HashOperator<'tree>,
    pub right: SimpleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <HashOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for HashExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HashType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<HashTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <HashTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for HashType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IfExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: SequenceExpression<'tree>,
    pub children: ::std::vec::Vec<IfExpressionChildren<'tree>>,
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
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <IfExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct IncludeModule<'tree> {
    pub span: ::treesitter_types::Span,
    pub module: ModuleExpression<'tree>,
    pub children: ::std::vec::Vec<IncludeModuleChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IncludeModule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "include_module");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("module", node))?;
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <IncludeModuleChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IncludeModule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IncludeModuleType<'tree> {
    pub span: ::treesitter_types::Span,
    pub module_type: ModuleType<'tree>,
    pub children: ::std::vec::Vec<IncludeModuleTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IncludeModuleType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "include_module_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module_type: {
                let child = node.child_by_field_name("module_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("module_type", node)
                })?;
                <ModuleType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <IncludeModuleTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IncludeModuleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IndexingOperatorPath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<IndexingOperatorPathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndexingOperatorPath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "indexing_operator_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <IndexingOperatorPathChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IndexingOperatorPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InfixExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub operator: InfixExpressionOperator<'tree>,
    pub right: Expression<'tree>,
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <InfixExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for InfixExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InheritanceDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<InheritanceDefinitionAlias<'tree>>,
    pub class: ClassExpression<'tree>,
    pub children: ::std::vec::Vec<ItemAttribute<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InheritanceDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inheritance_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: match node.child_by_field_name("alias") {
                Some(child) => Some(
                    <InheritanceDefinitionAlias as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            class: {
                let child = node
                    .child_by_field_name("class")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("class", node))?;
                <ClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(<ItemAttribute as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InheritanceDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InheritanceSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub class_type: SimpleClassType<'tree>,
    pub children: ::std::vec::Vec<ItemAttribute<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InheritanceSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inheritance_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            class_type: {
                let child = node.child_by_field_name("class_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("class_type", node)
                })?;
                <SimpleClassType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(<ItemAttribute as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InheritanceSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InstanceVariableDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<SequenceExpression<'tree>>,
    pub coercion: ::core::option::Option<Type<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<InstanceVariableDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceVariableDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance_variable_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            coercion: match node.child_by_field_name("coercion") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
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
                    items
                        .push(
                            <InstanceVariableDefinitionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InstanceVariableDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InstanceVariableExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ::core::option::Option<Expression<'tree>>,
    pub children: InstanceVariableName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceVariableExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance_variable_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: match node.child_by_field_name("expression") {
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
                <InstanceVariableName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InstanceVariableExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InstanceVariableSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<InstanceVariableSpecificationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceVariableSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance_variable_specification");
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
                    items
                        .push(
                            <InstanceVariableSpecificationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InstanceVariableSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InstantiatedClass<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InstantiatedClassChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstantiatedClass<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instantiated_class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <InstantiatedClassChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InstantiatedClass<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InstantiatedClassType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InstantiatedClassTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstantiatedClassType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instantiated_class_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <InstantiatedClassTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InstantiatedClassType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ItemAttribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ItemAttributeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ItemAttribute<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "item_attribute");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ItemAttributeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ItemAttribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ItemExtension<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ItemExtensionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ItemExtension<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "item_extension");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ItemExtensionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ItemExtension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ::core::option::Option<SimpleExpression<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: LabelName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_argument");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: match node.child_by_field_name("expression") {
                Some(child) => {
                    Some(<SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
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
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledArgumentType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: LabeledArgumentTypeType<'tree>,
    pub children: LabelName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledArgumentType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_argument_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <LabeledArgumentTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledArgumentType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledTupleElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ::core::option::Option<SimpleExpression<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: LabelName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledTupleElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_tuple_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: match node.child_by_field_name("expression") {
                Some(child) => {
                    Some(<SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
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
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledTupleElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledTupleElementBindingPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ::core::option::Option<SimpleBindingPattern<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: LabelName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledTupleElementBindingPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_tuple_element_binding_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(
                    <SimpleBindingPattern as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledTupleElementBindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledTupleElementPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ::core::option::Option<SimplePattern<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: LabelName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledTupleElementPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_tuple_element_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(<SimplePattern as ::treesitter_types::FromNode>::from_node(
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
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledTupleElementPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledTupleElementType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: SimpleType<'tree>,
    pub children: LabelName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledTupleElementType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_tuple_element_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <SimpleType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledTupleElementType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LazyExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SimpleExpression<'tree>,
    pub children: ::core::option::Option<AttributeId<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LazyExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lazy_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    Some(&child) => Some(<AttributeId as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for LazyExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LazyPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: LazyPatternPattern<'tree>,
    pub children: ::core::option::Option<AttributeId<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LazyPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lazy_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <LazyPatternPattern as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    Some(&child) => Some(<AttributeId as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for LazyPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetBinding<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<SequenceExpression<'tree>>,
    pub coercion: ::core::option::Option<Type<'tree>>,
    pub pattern: BindingPattern<'tree>,
    pub r#type: ::core::option::Option<PolymorphicType<'tree>>,
    pub children: ::std::vec::Vec<LetBindingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetBinding<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_binding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            coercion: match node.child_by_field_name("coercion") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <BindingPattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(<PolymorphicType as ::treesitter_types::FromNode>::from_node(child, src)?)
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
                        <LetBindingChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetBinding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetClassExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ClassExpression<'tree>,
    pub children: ValueDefinition<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetClassExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_class_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <ClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <ValueDefinition as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetClassExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetExceptionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SequenceExpression<'tree>,
    pub children: ExceptionDefinition<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetExceptionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_exception_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <ExceptionDefinition as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetExceptionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SequenceExpression<'tree>,
    pub children: ValueDefinition<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <ValueDefinition as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetModuleExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SequenceExpression<'tree>,
    pub children: ModuleDefinition<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetModuleExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_module_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <ModuleDefinition as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetModuleExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetOpenClassExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ClassExpression<'tree>,
    pub children: OpenModule<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetOpenClassExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_open_class_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <ClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <OpenModule as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetOpenClassExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetOpenClassType<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SimpleClassType<'tree>,
    pub children: OpenModule<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetOpenClassType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_open_class_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <SimpleClassType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <OpenModule as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetOpenClassType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetOpenExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SequenceExpression<'tree>,
    pub children: OpenModule<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetOpenExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_open_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <OpenModule as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetOpenExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListBindingPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BindingPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListBindingPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_binding_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<BindingPattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ListBindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for ListExpression<'_> {
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
pub struct LocalOpenExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ::core::option::Option<LocalOpenExpressionExpression<'tree>>,
    pub children: ModulePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalOpenExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_open_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: match node.child_by_field_name("expression") {
                Some(child) => Some(
                    <LocalOpenExpressionExpression as ::treesitter_types::FromNode>::from_node(
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
                <ModulePath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LocalOpenExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LocalOpenPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ::core::option::Option<LocalOpenPatternPattern<'tree>>,
    pub children: ModulePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalOpenPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_open_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(
                    <LocalOpenPatternPattern as ::treesitter_types::FromNode>::from_node(
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
                <ModulePath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LocalOpenPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LocalOpenType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: LocalOpenTypeType<'tree>,
    pub children: ExtendedModulePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalOpenType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_open_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <LocalOpenTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <ExtendedModulePath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LocalOpenType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: MatchCaseBody<'tree>,
    pub pattern: Pattern<'tree>,
    pub children: ::core::option::Option<Guard<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <MatchCaseBody as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for MatchCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SequenceExpression<'tree>,
    pub children: ::std::vec::Vec<MatchExpressionChildren<'tree>>,
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
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <MatchExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct MethodDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<SequenceExpression<'tree>>,
    pub r#type: ::core::option::Option<PolymorphicType<'tree>>,
    pub children: ::std::vec::Vec<MethodDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(<PolymorphicType as ::treesitter_types::FromNode>::from_node(child, src)?)
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
                        <MethodDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MethodInvocation<'tree> {
    pub span: ::treesitter_types::Span,
    pub method: MethodName<'tree>,
    pub object: SimpleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodInvocation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_invocation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            method: {
                let child = node
                    .child_by_field_name("method")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("method", node))?;
                <MethodName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodInvocation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MethodSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: PolymorphicType<'tree>,
    pub children: ::std::vec::Vec<MethodSpecificationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <PolymorphicType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <MethodSpecificationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MethodType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: PolymorphicType<'tree>,
    pub children: MethodName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <PolymorphicType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <MethodName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleApplication<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::core::option::Option<SimpleModuleExpression<'tree>>,
    pub functor: ModuleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleApplication<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_application");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: match node.child_by_field_name("argument") {
                Some(child) => Some(
                    <SimpleModuleExpression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            functor: {
                let child = node.child_by_field_name("functor").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("functor", node)
                })?;
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleApplication<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleBinding<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<ModuleExpression<'tree>>,
    pub module_type: ::core::option::Option<ModuleType<'tree>>,
    pub children: ::std::vec::Vec<ModuleBindingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleBinding<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_binding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            module_type: match node.child_by_field_name("module_type") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
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
                        <ModuleBindingChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleBinding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModuleDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ModuleDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub module_type: ::core::option::Option<ModuleType<'tree>>,
    pub children: ::core::option::Option<ModuleName<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module_type: match node.child_by_field_name("module_type") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
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
                    Some(&child) => Some(<ModuleName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModulePath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModulePathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModulePath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ModulePathChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModulePath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleTypeConstraint<'tree> {
    pub span: ::treesitter_types::Span,
    pub module_type: ModuleType<'tree>,
    pub children: ::std::vec::Vec<ModuleTypeConstraintChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypeConstraint<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_type_constraint");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module_type: {
                let child = node.child_by_field_name("module_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("module_type", node)
                })?;
                <ModuleType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ModuleTypeConstraintChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleTypeConstraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleTypeDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<ModuleType<'tree>>,
    pub children: ::std::vec::Vec<ModuleTypeDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypeDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_type_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
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
                        <ModuleTypeDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleTypeDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleTypeOf<'tree> {
    pub span: ::treesitter_types::Span,
    pub module: ModuleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypeOf<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_type_of");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("module", node))?;
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleTypeOf<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleTypePath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModuleTypePathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypePath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_type_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ModuleTypePathChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleTypePath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NewExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NewExpressionChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <NewExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct Number<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Number<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct ObjectCopyExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InstanceVariableExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectCopyExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_copy_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <InstanceVariableExpression as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ObjectCopyExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ObjectExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub self_: ::core::option::Option<Pattern<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<ObjectExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            self_: match node.child_by_field_name("self") {
                Some(child) => Some(<Pattern as ::treesitter_types::FromNode>::from_node(
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
                        <ObjectExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ObjectExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ObjectType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ObjectTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ObjectTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ObjectType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OpenModule<'tree> {
    pub span: ::treesitter_types::Span,
    pub module: ModuleExpression<'tree>,
    pub children: ::std::vec::Vec<OpenModuleChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OpenModule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "open_module");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("module", node))?;
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <OpenModuleChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for OpenModule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OrPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<OrPatternChildren<'tree>>,
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
                    items.push(
                        <OrPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
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
#[derive(Debug, Clone)]
pub struct PackageExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub module: ModuleExpression<'tree>,
    pub module_type: ::core::option::Option<ModuleType<'tree>>,
    pub children: ::core::option::Option<AttributeId<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("module", node))?;
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            module_type: match node.child_by_field_name("module_type") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
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
                    Some(&child) => Some(<AttributeId as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for PackageExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PackagePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub module_type: ::core::option::Option<ModuleType<'tree>>,
    pub children: ::std::vec::Vec<PackagePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackagePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module_type: match node.child_by_field_name("module_type") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
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
                        <PackagePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PackagePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PackageType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PackageTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <PackageTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PackageType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PackedModule<'tree> {
    pub span: ::treesitter_types::Span,
    pub coercion: ::core::option::Option<ModuleType<'tree>>,
    pub module_type: ::core::option::Option<ModuleType<'tree>>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackedModule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "packed_module");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            coercion: match node.child_by_field_name("coercion") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            module_type: match node.child_by_field_name("module_type") {
                Some(child) => Some(<ModuleType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PackedModule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Parameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub default: ::core::option::Option<SequenceExpression<'tree>>,
    pub pattern: ParameterPattern<'tree>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::core::option::Option<LabelName<'tree>>,
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
            default: match node.child_by_field_name("default") {
                Some(child) => Some(
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <ParameterPattern as ::treesitter_types::FromNode>::from_node(child, src)?
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
                match non_field_children.first() {
                    Some(&child) => Some(<LabelName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
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
#[derive(Debug, Clone)]
pub struct ParenthesizedClassExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ClassExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedClassExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_class_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <ClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedClassExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SequenceExpression<'tree>,
    pub children: ::core::option::Option<AttributeId<'tree>>,
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
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    Some(&child) => Some(<AttributeId as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
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
pub struct ParenthesizedModuleExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ModuleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedModuleExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_module_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedModuleExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedModuleType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ModuleType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedModuleType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_module_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <ModuleType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedModuleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ParenthesizedOperatorChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedOperator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <ParenthesizedOperatorChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ParenthesizedPatternChildren<'tree>,
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
                <ParenthesizedPatternChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ParenthesizedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PolymorphicType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<PolymorphicTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PolymorphicType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "polymorphic_type");
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
                        <PolymorphicTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PolymorphicType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PolymorphicVariantPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeConstructorPath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PolymorphicVariantPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "polymorphic_variant_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <TypeConstructorPath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PolymorphicVariantPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PolymorphicVariantType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PolymorphicVariantTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PolymorphicVariantType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "polymorphic_variant_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <PolymorphicVariantTypeChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PolymorphicVariantType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PrefixExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SimpleExpression<'tree>,
    pub operator: PrefixOperator<'tree>,
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
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <PrefixOperator as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct QuotedExtension<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<QuotedExtensionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedExtension<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_extension");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <QuotedExtensionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedExtension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QuotedItemExtension<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<QuotedItemExtensionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedItemExtension<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_item_extension");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <QuotedItemExtensionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedItemExtension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QuotedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<QuotedStringContent<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <QuotedStringContent as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QuotedStringContent<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<QuotedStringContentChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedStringContent<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_string_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <QuotedStringContentChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedStringContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RangePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: SignedConstant<'tree>,
    pub right: SignedConstant<'tree>,
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
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <SignedConstant as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <SignedConstant as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RangePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RecordBindingPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordBindingPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record_binding_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<FieldPattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecordBindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RecordDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldDeclaration<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <FieldDeclaration as ::treesitter_types::FromNode>::from_node(child, src)?,
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
pub struct RecordExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub record: ::core::option::Option<SimpleExpression<'tree>>,
    pub children: ::std::vec::Vec<FieldExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            record: match node.child_by_field_name("record") {
                Some(child) => {
                    Some(<SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?)
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
                        <FieldExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecordExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RecordPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<FieldPattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecordPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RefutationCase<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RefutationCase<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "refutation_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RefutationCase<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RefutationCase<'_> {
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
pub struct SetExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Expression<'tree>,
    pub children: SetExpressionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "set_expression");
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
                <SetExpressionChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SetExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SignExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: Expression<'tree>,
    pub operator: SignOperator<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SignExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sign_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <SignOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SignExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SignOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SignOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sign_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SignOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SignOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Signature<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SignatureItem<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Signature<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "signature");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<SignatureItem as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Signature<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SignedNumber<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SignedNumber<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "signed_number");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SignedNumber<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SignedNumber<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct String<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<StringContent<'tree>>,
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
                match non_field_children.first() {
                    Some(&child) => Some(
                        <StringContent as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
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
pub struct StringGetExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub index: SequenceExpression<'tree>,
    pub operator: ::core::option::Option<IndexingOperatorPath<'tree>>,
    pub string: SimpleExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringGetExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_get_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("index", node))?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => Some(
                    <IndexingOperatorPath as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            string: {
                let child = node
                    .child_by_field_name("string")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("string", node))?;
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for StringGetExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Structure<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StructureChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Structure<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "structure");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <StructureChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Structure<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Tag<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Tag<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Tag<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Tag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TagPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: TagPatternPattern<'tree>,
    pub children: Tag<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <TagPatternPattern as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <Tag as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TagPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TagSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TagSpecificationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TagSpecificationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TagSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ThenClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThenClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "then_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ThenClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ToplevelDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ToplevelDirectiveChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ToplevelDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "toplevel_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ToplevelDirectiveChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ToplevelDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SequenceExpression<'tree>,
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
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(
                        <TupleExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
pub struct TypeBinding<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<TypeBindingBody<'tree>>,
    pub equation: ::core::option::Option<Type<'tree>>,
    pub name: TypeBindingName<'tree>,
    pub children: ::std::vec::Vec<TypeBindingChildren<'tree>>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<TypeBindingBody as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            equation: match node.child_by_field_name("equation") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <TypeBindingName as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <TypeBindingChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeBinding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeConstraint<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: Type<'tree>,
    pub r#type: Type<'tree>,
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
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for TypeConstraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeConstructorPath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeConstructorPathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeConstructorPath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_constructor_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TypeConstructorPathChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeConstructorPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeDefinition<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct TypeParameterConstraint<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: Type<'tree>,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<ItemAttribute<'tree>>,
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
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(<ItemAttribute as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
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
pub struct TypeVariable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeVariable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_variable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TypeVariable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TypeVariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypedClassExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub class: ClassExpression<'tree>,
    pub class_type: ClassType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedClassExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typed_class_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            class: {
                let child = node
                    .child_by_field_name("class")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("class", node))?;
                <ClassExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            class_type: {
                let child = node.child_by_field_name("class_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("class_type", node)
                })?;
                <ClassType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypedClassExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SequenceExpression<'tree>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typed_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for TypedExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypedModuleExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub module: ModuleExpression<'tree>,
    pub module_type: ModuleType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedModuleExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typed_module_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("module", node))?;
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            module_type: {
                let child = node.child_by_field_name("module_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("module_type", node)
                })?;
                <ModuleType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypedModuleExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: TypedPatternPattern<'tree>,
    pub r#type: Type<'tree>,
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
                <TypedPatternPattern as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ValueDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ValueDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValueDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "value_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ValueDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ValueDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ValuePath<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ValuePathChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValuePath<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "value_path");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ValuePathChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ValuePath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ValueSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: PolymorphicType<'tree>,
    pub children: ::std::vec::Vec<ValueSpecificationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValueSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "value_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <PolymorphicType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ValueSpecificationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ValueSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariantDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConstructorDeclaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariantDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variant_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariantDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhileExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: SequenceExpression<'tree>,
    pub children: ::std::vec::Vec<WhileExpressionChildren<'tree>>,
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
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <WhileExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
pub struct AndOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AndOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "and_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AndOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AndOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AssignOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assign_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AssignOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AssignOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ClassName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ClassName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassTypeName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassTypeName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_type_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ClassTypeName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ClassTypeName<'_> {
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
pub struct ConcatOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConcatOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "concat_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ConcatOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ConcatOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstructorName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ConstructorName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ConstructorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConversionSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConversionSpecification<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "conversion_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ConversionSpecification<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ConversionSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FieldName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FieldName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HashOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HashOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HashOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IndexingOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndexingOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "indexing_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IndexingOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IndexingOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InstanceVariableName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceVariableName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance_variable_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InstanceVariableName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InstanceVariableName<'_> {
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
pub struct LetAndOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetAndOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_and_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LetAndOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LetAndOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LetOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LetOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LetOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LineNumberDirective<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LineNumberDirective<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "line_number_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LineNumberDirective<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LineNumberDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MatchOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MatchOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MethodName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MethodName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MethodName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ModuleName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ModuleName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleTypeName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypeName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_type_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ModuleTypeName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ModuleTypeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MultOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MultOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "mult_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MultOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MultOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OcamlyaccValue<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OcamlyaccValue<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ocamlyacc_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OcamlyaccValue<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OcamlyaccValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OrOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OrOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "or_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OrOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OrOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PowOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PowOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pow_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PowOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PowOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PrefixOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrefixOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrefixOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PrettyPrintingIndication<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrettyPrintingIndication<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pretty_printing_indication");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrettyPrintingIndication<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrettyPrintingIndication<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RelOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RelOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rel_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RelOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RelOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct TypeConstructor<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeConstructor<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_constructor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TypeConstructor<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TypeConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ValueName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValueName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "value_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ValueName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ValueName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ValuePattern<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValuePattern<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "value_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ValuePattern<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ValuePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum AliasPatternAlias<'tree> {
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    ValueName(::std::boxed::Box<ValueName<'tree>>),
    ValuePattern(::std::boxed::Box<ValuePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasPatternAlias<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_name" => Ok(Self::ValueName(::std::boxed::Box::new(
                <ValueName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_pattern" => Ok(Self::ValuePattern(::std::boxed::Box::new(
                <ValuePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AliasPatternAlias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ValueName(inner) => inner.span(),
            Self::ValuePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AliasPatternPattern<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for AliasPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ApplicationExpressionArgument<'tree> {
    SimpleExpression(::std::boxed::Box<SimpleExpression<'tree>>),
    LabeledArgument(::std::boxed::Box<LabeledArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ApplicationExpressionArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_argument" => Ok(Self::LabeledArgument(::std::boxed::Box::new(
                <LabeledArgument as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SimpleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimpleExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ApplicationExpressionArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleExpression(inner) => inner.span(),
            Self::LabeledArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrayPatternChildren<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ArrayPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributeChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    AttributePayload(::std::boxed::Box<AttributePayload<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_payload" => Ok(Self::AttributePayload(::std::boxed::Box::new(
                <AttributePayload as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::AttributePayload(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributePayloadChildren<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    SignatureItem(::std::boxed::Box<SignatureItem<'tree>>),
    StructureItem(::std::boxed::Box<StructureItem<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    ExpressionItem(::std::boxed::Box<ExpressionItem<'tree>>),
    Guard(::std::boxed::Box<Guard<'tree>>),
    ToplevelDirective(::std::boxed::Box<ToplevelDirective<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributePayloadChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_item" => Ok(Self::ExpressionItem(::std::boxed::Box::new(
                <ExpressionItem as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard" => Ok(Self::Guard(::std::boxed::Box::new(
                <Guard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "toplevel_directive" => Ok(Self::ToplevelDirective(::std::boxed::Box::new(
                <ToplevelDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <SignatureItem as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::SignatureItem(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <StructureItem as ::treesitter_types::FromNode>::from_node(node, src)
                        {
                            Ok(Self::StructureItem(::std::boxed::Box::new(v)))
                        } else {
                            if let Ok(v) =
                                <Type as ::treesitter_types::FromNode>::from_node(node, src)
                            {
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
    }
}
impl ::treesitter_types::Spanned for AttributePayloadChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::SignatureItem(inner) => inner.span(),
            Self::StructureItem(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::ExpressionItem(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
            Self::ToplevelDirective(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassApplicationArgument<'tree> {
    SimpleExpression(::std::boxed::Box<SimpleExpression<'tree>>),
    LabeledArgument(::std::boxed::Box<LabeledArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassApplicationArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_argument" => Ok(Self::LabeledArgument(::std::boxed::Box::new(
                <LabeledArgument as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SimpleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SimpleExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ClassApplicationArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleExpression(inner) => inner.span(),
            Self::LabeledArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassBindingChildren<'tree> {
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    ClassName(::std::boxed::Box<ClassName<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    TypeVariable(::std::boxed::Box<TypeVariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassBindingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_name" => Ok(Self::ClassName(::std::boxed::Box::new(
                <ClassName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_variable" => Ok(Self::TypeVariable(::std::boxed::Box::new(
                <TypeVariable as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Parameter as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Parameter(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ClassBindingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Parameter(inner) => inner.span(),
            Self::ClassName(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::TypeVariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassBodyTypeChildren<'tree> {
    ClassFieldSpecification(::std::boxed::Box<ClassFieldSpecification<'tree>>),
    FloatingAttribute(::std::boxed::Box<FloatingAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassBodyTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "floating_attribute" => Ok(Self::FloatingAttribute(::std::boxed::Box::new(
                <FloatingAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <ClassFieldSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::ClassFieldSpecification(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ClassBodyTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassFieldSpecification(inner) => inner.span(),
            Self::FloatingAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassDefinitionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ClassBinding(::std::boxed::Box<ClassBinding<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_binding" => Ok(Self::ClassBinding(::std::boxed::Box::new(
                <ClassBinding as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ClassBinding(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassFunctionTypeDomain<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    LabeledArgumentType(::std::boxed::Box<LabeledArgumentType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassFunctionTypeDomain<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_argument_type" => Ok(Self::LabeledArgumentType(::std::boxed::Box::new(
                <LabeledArgumentType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ClassFunctionTypeDomain<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::LabeledArgumentType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassPathChildren<'tree> {
    ClassName(::std::boxed::Box<ClassName<'tree>>),
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassPathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_name" => Ok(Self::ClassName(::std::boxed::Box::new(
                <ClassName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassPathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassName(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassTypeBindingChildren<'tree> {
    ClassTypeName(::std::boxed::Box<ClassTypeName<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    TypeVariable(::std::boxed::Box<TypeVariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassTypeBindingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_type_name" => Ok(Self::ClassTypeName(::std::boxed::Box::new(
                <ClassTypeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_variable" => Ok(Self::TypeVariable(::std::boxed::Box::new(
                <TypeVariable as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassTypeBindingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassTypeName(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::TypeVariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassTypeDefinitionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ClassTypeBinding(::std::boxed::Box<ClassTypeBinding<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassTypeDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_type_binding" => Ok(Self::ClassTypeBinding(::std::boxed::Box::new(
                <ClassTypeBinding as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassTypeDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ClassTypeBinding(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassTypePathChildren<'tree> {
    ClassTypeName(::std::boxed::Box<ClassTypeName<'tree>>),
    ExtendedModulePath(::std::boxed::Box<ExtendedModulePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassTypePathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_type_name" => Ok(Self::ClassTypeName(::std::boxed::Box::new(
                <ClassTypeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extended_module_path" => Ok(Self::ExtendedModulePath(::std::boxed::Box::new(
                <ExtendedModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassTypePathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassTypeName(inner) => inner.span(),
            Self::ExtendedModulePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompilationUnitChildren<'tree> {
    StructureItem(::std::boxed::Box<StructureItem<'tree>>),
    ExpressionItem(::std::boxed::Box<ExpressionItem<'tree>>),
    Shebang(::std::boxed::Box<Shebang<'tree>>),
    ToplevelDirective(::std::boxed::Box<ToplevelDirective<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompilationUnitChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_item" => Ok(Self::ExpressionItem(::std::boxed::Box::new(
                <ExpressionItem as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "shebang" => Ok(Self::Shebang(::std::boxed::Box::new(
                <Shebang as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "toplevel_directive" => Ok(Self::ToplevelDirective(::std::boxed::Box::new(
                <ToplevelDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <StructureItem as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::StructureItem(::std::boxed::Box::new(v)))
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
            Self::StructureItem(inner) => inner.span(),
            Self::ExpressionItem(inner) => inner.span(),
            Self::Shebang(inner) => inner.span(),
            Self::ToplevelDirective(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConsPatternLeft<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConsPatternLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ConsPatternLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConsPatternRight<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConsPatternRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ConsPatternRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstrainTypeChildren<'tree> {
    TypeConstraint(::std::boxed::Box<TypeConstraint<'tree>>),
    TypeConstructorPath(::std::boxed::Box<TypeConstructorPath<'tree>>),
    TypeVariable(::std::boxed::Box<TypeVariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstrainTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_constraint" => Ok(Self::TypeConstraint(::std::boxed::Box::new(
                <TypeConstraint as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_constructor_path" => Ok(Self::TypeConstructorPath(::std::boxed::Box::new(
                <TypeConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_variable" => Ok(Self::TypeVariable(::std::boxed::Box::new(
                <TypeVariable as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstrainTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TypeConstraint(inner) => inner.span(),
            Self::TypeConstructorPath(inner) => inner.span(),
            Self::TypeVariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstructorDeclarationChildren<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ConstructorName(::std::boxed::Box<ConstructorName<'tree>>),
    ConstructorPath(::std::boxed::Box<ConstructorPath<'tree>>),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor_name" => Ok(Self::ConstructorName(::std::boxed::Box::new(
                <ConstructorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_path" => Ok(Self::ConstructorPath(::std::boxed::Box::new(
                <ConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ConstructorName(inner) => inner.span(),
            Self::ConstructorPath(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstructorPathChildren<'tree> {
    ConstructorName(::std::boxed::Box<ConstructorName<'tree>>),
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorPathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor_name" => Ok(Self::ConstructorName(::std::boxed::Box::new(
                <ConstructorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorPathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstructorName(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstructorPatternPattern<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstructorPatternChildren<'tree> {
    AbstractType(::std::boxed::Box<AbstractType<'tree>>),
    ConstructorPath(::std::boxed::Box<ConstructorPath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_type" => Ok(Self::AbstractType(::std::boxed::Box::new(
                <AbstractType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_path" => Ok(Self::ConstructorPath(::std::boxed::Box::new(
                <ConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractType(inner) => inner.span(),
            Self::ConstructorPath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExceptionDefinitionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ConstructorDeclaration(::std::boxed::Box<ConstructorDeclaration<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExceptionDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_declaration" => Ok(Self::ConstructorDeclaration(::std::boxed::Box::new(
                <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExceptionDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ConstructorDeclaration(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpressionItemChildren<'tree> {
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SequenceExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ExpressionItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SequenceExpression(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExtendedModulePathChildren<'tree> {
    ExtendedModulePath(::std::boxed::Box<ExtendedModulePath<'tree>>),
    ModuleName(::std::boxed::Box<ModuleName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtendedModulePathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "extended_module_path" => Ok(Self::ExtendedModulePath(::std::boxed::Box::new(
                <ExtendedModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_name" => Ok(Self::ModuleName(::std::boxed::Box::new(
                <ModuleName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExtendedModulePathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExtendedModulePath(inner) => inner.span(),
            Self::ModuleName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExtensionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    AttributePayload(::std::boxed::Box<AttributePayload<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtensionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_payload" => Ok(Self::AttributePayload(::std::boxed::Box::new(
                <AttributePayload as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExtensionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::AttributePayload(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExternalChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ValueName(::std::boxed::Box<ValueName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExternalChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_name" => Ok(Self::ValueName(::std::boxed::Box::new(
                <ValueName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExternalChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ValueName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldPathChildren<'tree> {
    FieldName(::std::boxed::Box<FieldName<'tree>>),
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_name" => Ok(Self::FieldName(::std::boxed::Box::new(
                <FieldName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldPathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldName(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldPatternPattern<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for FieldPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FloatingAttributeChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    AttributePayload(::std::boxed::Box<AttributePayload<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FloatingAttributeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_payload" => Ok(Self::AttributePayload(::std::boxed::Box::new(
                <AttributePayload as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FloatingAttributeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::AttributePayload(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForExpressionName<'tree> {
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    ValuePattern(::std::boxed::Box<ValuePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_pattern" => Ok(Self::ValuePattern(::std::boxed::Box::new(
                <ValuePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ValuePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForExpressionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    DoClause(::std::boxed::Box<DoClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_clause" => Ok(Self::DoClause(::std::boxed::Box::new(
                <DoClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::DoClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunExpressionChildren<'tree> {
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Parameter as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Parameter(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FunExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Parameter(inner) => inner.span(),
            Self::AttributeId(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionExpressionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    MatchCase(::std::boxed::Box<MatchCase<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_case" => Ok(Self::MatchCase(::std::boxed::Box::new(
                <MatchCase as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::MatchCase(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionTypeDomain<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    LabeledArgumentType(::std::boxed::Box<LabeledArgumentType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionTypeDomain<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_argument_type" => Ok(Self::LabeledArgumentType(::std::boxed::Box::new(
                <LabeledArgumentType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FunctionTypeDomain<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::LabeledArgumentType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum HashTypeChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    ClassTypePath(::std::boxed::Box<ClassTypePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_type_path" => Ok(Self::ClassTypePath(::std::boxed::Box::new(
                <ClassTypePath as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for HashTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::ClassTypePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfExpressionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ElseClause(::std::boxed::Box<ElseClause<'tree>>),
    ThenClause(::std::boxed::Box<ThenClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "else_clause" => Ok(Self::ElseClause(::std::boxed::Box::new(
                <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "then_clause" => Ok(Self::ThenClause(::std::boxed::Box::new(
                <ThenClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IfExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::ThenClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IncludeModuleChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IncludeModuleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IncludeModuleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IncludeModuleTypeChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IncludeModuleTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IncludeModuleTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IndexingOperatorPathChildren<'tree> {
    IndexingOperator(::std::boxed::Box<IndexingOperator<'tree>>),
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndexingOperatorPathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indexing_operator" => Ok(Self::IndexingOperator(::std::boxed::Box::new(
                <IndexingOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IndexingOperatorPathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::IndexingOperator(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixExpressionOperator<'tree> {
    AddOperator(::std::boxed::Box<AddOperator<'tree>>),
    AndOperator(::std::boxed::Box<AndOperator<'tree>>),
    AssignOperator(::std::boxed::Box<AssignOperator<'tree>>),
    ConcatOperator(::std::boxed::Box<ConcatOperator<'tree>>),
    MultOperator(::std::boxed::Box<MultOperator<'tree>>),
    OrOperator(::std::boxed::Box<OrOperator<'tree>>),
    PowOperator(::std::boxed::Box<PowOperator<'tree>>),
    RelOperator(::std::boxed::Box<RelOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixExpressionOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "add_operator" => Ok(Self::AddOperator(::std::boxed::Box::new(
                <AddOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "and_operator" => Ok(Self::AndOperator(::std::boxed::Box::new(
                <AndOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "assign_operator" => Ok(Self::AssignOperator(::std::boxed::Box::new(
                <AssignOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concat_operator" => Ok(Self::ConcatOperator(::std::boxed::Box::new(
                <ConcatOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "mult_operator" => Ok(Self::MultOperator(::std::boxed::Box::new(
                <MultOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "or_operator" => Ok(Self::OrOperator(::std::boxed::Box::new(
                <OrOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pow_operator" => Ok(Self::PowOperator(::std::boxed::Box::new(
                <PowOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "rel_operator" => Ok(Self::RelOperator(::std::boxed::Box::new(
                <RelOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixExpressionOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AddOperator(inner) => inner.span(),
            Self::AndOperator(inner) => inner.span(),
            Self::AssignOperator(inner) => inner.span(),
            Self::ConcatOperator(inner) => inner.span(),
            Self::MultOperator(inner) => inner.span(),
            Self::OrOperator(inner) => inner.span(),
            Self::PowOperator(inner) => inner.span(),
            Self::RelOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InheritanceDefinitionAlias<'tree> {
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    ValuePattern(::std::boxed::Box<ValuePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InheritanceDefinitionAlias<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_pattern" => Ok(Self::ValuePattern(::std::boxed::Box::new(
                <ValuePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InheritanceDefinitionAlias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ValuePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InstanceVariableDefinitionChildren<'tree> {
    InstanceVariableName(::std::boxed::Box<InstanceVariableName<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceVariableDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "instance_variable_name" => Ok(Self::InstanceVariableName(::std::boxed::Box::new(
                <InstanceVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InstanceVariableDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::InstanceVariableName(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InstanceVariableSpecificationChildren<'tree> {
    InstanceVariableName(::std::boxed::Box<InstanceVariableName<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceVariableSpecificationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "instance_variable_name" => Ok(Self::InstanceVariableName(::std::boxed::Box::new(
                <InstanceVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InstanceVariableSpecificationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::InstanceVariableName(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InstantiatedClassChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    ClassPath(::std::boxed::Box<ClassPath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstantiatedClassChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_path" => Ok(Self::ClassPath(::std::boxed::Box::new(
                <ClassPath as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for InstantiatedClassChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::ClassPath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InstantiatedClassTypeChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    ClassTypePath(::std::boxed::Box<ClassTypePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstantiatedClassTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_type_path" => Ok(Self::ClassTypePath(::std::boxed::Box::new(
                <ClassTypePath as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for InstantiatedClassTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::ClassTypePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ItemAttributeChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    AttributePayload(::std::boxed::Box<AttributePayload<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ItemAttributeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_payload" => Ok(Self::AttributePayload(::std::boxed::Box::new(
                <AttributePayload as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ItemAttributeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::AttributePayload(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ItemExtensionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    AttributePayload(::std::boxed::Box<AttributePayload<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ItemExtensionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_payload" => Ok(Self::AttributePayload(::std::boxed::Box::new(
                <AttributePayload as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ItemExtensionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::AttributePayload(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LabeledArgumentTypeType<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    LabeledArgumentType(::std::boxed::Box<LabeledArgumentType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledArgumentTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_argument_type" => Ok(Self::LabeledArgumentType(::std::boxed::Box::new(
                <LabeledArgumentType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for LabeledArgumentTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::LabeledArgumentType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LazyPatternPattern<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LazyPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for LazyPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LetBindingChildren<'tree> {
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetBindingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Parameter as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Parameter(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for LetBindingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Parameter(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListPatternChildren<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ListPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LocalOpenExpressionExpression<'tree> {
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
    ArrayExpression(::std::boxed::Box<ArrayExpression<'tree>>),
    ListExpression(::std::boxed::Box<ListExpression<'tree>>),
    ObjectCopyExpression(::std::boxed::Box<ObjectCopyExpression<'tree>>),
    PackageExpression(::std::boxed::Box<PackageExpression<'tree>>),
    RecordExpression(::std::boxed::Box<RecordExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalOpenExpressionExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_expression" => Ok(Self::ArrayExpression(::std::boxed::Box::new(
                <ArrayExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_expression" => Ok(Self::ListExpression(::std::boxed::Box::new(
                <ListExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_copy_expression" => Ok(Self::ObjectCopyExpression(::std::boxed::Box::new(
                <ObjectCopyExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "package_expression" => Ok(Self::PackageExpression(::std::boxed::Box::new(
                <PackageExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_expression" => Ok(Self::RecordExpression(::std::boxed::Box::new(
                <RecordExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SequenceExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for LocalOpenExpressionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SequenceExpression(inner) => inner.span(),
            Self::ArrayExpression(inner) => inner.span(),
            Self::ListExpression(inner) => inner.span(),
            Self::ObjectCopyExpression(inner) => inner.span(),
            Self::PackageExpression(inner) => inner.span(),
            Self::RecordExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LocalOpenPatternPattern<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    ArrayBindingPattern(::std::boxed::Box<ArrayBindingPattern<'tree>>),
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    ListBindingPattern(::std::boxed::Box<ListBindingPattern<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    RecordBindingPattern(::std::boxed::Box<RecordBindingPattern<'tree>>),
    RecordPattern(::std::boxed::Box<RecordPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalOpenPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_binding_pattern" => Ok(Self::ArrayBindingPattern(::std::boxed::Box::new(
                <ArrayBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_binding_pattern" => Ok(Self::ListBindingPattern(::std::boxed::Box::new(
                <ListBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_binding_pattern" => Ok(Self::RecordBindingPattern(::std::boxed::Box::new(
                <RecordBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_pattern" => Ok(Self::RecordPattern(::std::boxed::Box::new(
                <RecordPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for LocalOpenPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::ArrayBindingPattern(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::ListBindingPattern(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::RecordBindingPattern(inner) => inner.span(),
            Self::RecordPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LocalOpenTypeType<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    PackageType(::std::boxed::Box<PackageType<'tree>>),
    PolymorphicVariantType(::std::boxed::Box<PolymorphicVariantType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalOpenTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "package_type" => Ok(Self::PackageType(::std::boxed::Box::new(
                <PackageType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "polymorphic_variant_type" => Ok(Self::PolymorphicVariantType(::std::boxed::Box::new(
                <PolymorphicVariantType as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for LocalOpenTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::PackageType(inner) => inner.span(),
            Self::PolymorphicVariantType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MatchCaseBody<'tree> {
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
    RefutationCase(::std::boxed::Box<RefutationCase<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchCaseBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "refutation_case" => Ok(Self::RefutationCase(::std::boxed::Box::new(
                <RefutationCase as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::SequenceExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for MatchCaseBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SequenceExpression(inner) => inner.span(),
            Self::RefutationCase(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MatchExpressionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    MatchCase(::std::boxed::Box<MatchCase<'tree>>),
    MatchOperator(::std::boxed::Box<MatchOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_case" => Ok(Self::MatchCase(::std::boxed::Box::new(
                <MatchCase as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_operator" => Ok(Self::MatchOperator(::std::boxed::Box::new(
                <MatchOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MatchExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::MatchCase(inner) => inner.span(),
            Self::MatchOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodDefinitionChildren<'tree> {
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    MethodName(::std::boxed::Box<MethodName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_name" => Ok(Self::MethodName(::std::boxed::Box::new(
                <MethodName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Parameter as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Parameter(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for MethodDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Parameter(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::MethodName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodSpecificationChildren<'tree> {
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    MethodName(::std::boxed::Box<MethodName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodSpecificationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_name" => Ok(Self::MethodName(::std::boxed::Box::new(
                <MethodName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MethodSpecificationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ItemAttribute(inner) => inner.span(),
            Self::MethodName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleBindingChildren<'tree> {
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    ModuleName(::std::boxed::Box<ModuleName<'tree>>),
    ModuleParameter(::std::boxed::Box<ModuleParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleBindingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_name" => Ok(Self::ModuleName(::std::boxed::Box::new(
                <ModuleName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_parameter" => Ok(Self::ModuleParameter(::std::boxed::Box::new(
                <ModuleParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleBindingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ItemAttribute(inner) => inner.span(),
            Self::ModuleName(inner) => inner.span(),
            Self::ModuleParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleDefinitionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ModuleBinding(::std::boxed::Box<ModuleBinding<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_binding" => Ok(Self::ModuleBinding(::std::boxed::Box::new(
                <ModuleBinding as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ModuleBinding(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModulePathChildren<'tree> {
    ModuleName(::std::boxed::Box<ModuleName<'tree>>),
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModulePathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "module_name" => Ok(Self::ModuleName(::std::boxed::Box::new(
                <ModuleName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModulePathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ModuleName(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleTypeConstraintChildren<'tree> {
    ConstrainModule(::std::boxed::Box<ConstrainModule<'tree>>),
    ConstrainModuleType(::std::boxed::Box<ConstrainModuleType<'tree>>),
    ConstrainType(::std::boxed::Box<ConstrainType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypeConstraintChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constrain_module" => Ok(Self::ConstrainModule(::std::boxed::Box::new(
                <ConstrainModule as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constrain_module_type" => Ok(Self::ConstrainModuleType(::std::boxed::Box::new(
                <ConstrainModuleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constrain_type" => Ok(Self::ConstrainType(::std::boxed::Box::new(
                <ConstrainType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleTypeConstraintChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstrainModule(inner) => inner.span(),
            Self::ConstrainModuleType(inner) => inner.span(),
            Self::ConstrainType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleTypeDefinitionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    ModuleTypeName(::std::boxed::Box<ModuleTypeName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypeDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_type_name" => Ok(Self::ModuleTypeName(::std::boxed::Box::new(
                <ModuleTypeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleTypeDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::ModuleTypeName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleTypePathChildren<'tree> {
    ExtendedModulePath(::std::boxed::Box<ExtendedModulePath<'tree>>),
    ModuleTypeName(::std::boxed::Box<ModuleTypeName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleTypePathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "extended_module_path" => Ok(Self::ExtendedModulePath(::std::boxed::Box::new(
                <ExtendedModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_type_name" => Ok(Self::ModuleTypeName(::std::boxed::Box::new(
                <ModuleTypeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleTypePathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExtendedModulePath(inner) => inner.span(),
            Self::ModuleTypeName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NewExpressionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ClassPath(::std::boxed::Box<ClassPath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_path" => Ok(Self::ClassPath(::std::boxed::Box::new(
                <ClassPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NewExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ClassPath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ObjectExpressionChildren<'tree> {
    ClassField(::std::boxed::Box<ClassField<'tree>>),
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    FloatingAttribute(::std::boxed::Box<FloatingAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_attribute" => Ok(Self::FloatingAttribute(::std::boxed::Box::new(
                <FloatingAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <ClassField as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::ClassField(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ObjectExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassField(inner) => inner.span(),
            Self::AttributeId(inner) => inner.span(),
            Self::FloatingAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ObjectTypeChildren<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    MethodType(::std::boxed::Box<MethodType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "method_type" => Ok(Self::MethodType(::std::boxed::Box::new(
                <MethodType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ObjectTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::MethodType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OpenModuleChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OpenModuleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OpenModuleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OrPatternChildren<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OrPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for OrPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PackagePatternChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ModuleName(::std::boxed::Box<ModuleName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackagePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_name" => Ok(Self::ModuleName(::std::boxed::Box::new(
                <ModuleName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PackagePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ModuleName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PackageTypeChildren<'tree> {
    ModuleType(::std::boxed::Box<ModuleType<'tree>>),
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <ModuleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::ModuleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PackageTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ModuleType(inner) => inner.span(),
            Self::AttributeId(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    SimplePattern(::std::boxed::Box<SimplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::Pattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <SimplePattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::SimplePattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ParameterPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::SimplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedOperatorChildren<'tree> {
    InfixOperator(::std::boxed::Box<InfixOperator<'tree>>),
    HashOperator(::std::boxed::Box<HashOperator<'tree>>),
    IndexingOperator(::std::boxed::Box<IndexingOperator<'tree>>),
    LetAndOperator(::std::boxed::Box<LetAndOperator<'tree>>),
    LetOperator(::std::boxed::Box<LetOperator<'tree>>),
    MatchOperator(::std::boxed::Box<MatchOperator<'tree>>),
    PrefixOperator(::std::boxed::Box<PrefixOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedOperatorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "hash_operator" => Ok(Self::HashOperator(::std::boxed::Box::new(
                <HashOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indexing_operator" => Ok(Self::IndexingOperator(::std::boxed::Box::new(
                <IndexingOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_and_operator" => Ok(Self::LetAndOperator(::std::boxed::Box::new(
                <LetAndOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_operator" => Ok(Self::LetOperator(::std::boxed::Box::new(
                <LetOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_operator" => Ok(Self::MatchOperator(::std::boxed::Box::new(
                <MatchOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "prefix_operator" => Ok(Self::PrefixOperator(::std::boxed::Box::new(
                <PrefixOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <InfixOperator as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::InfixOperator(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ParenthesizedOperatorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::InfixOperator(inner) => inner.span(),
            Self::HashOperator(inner) => inner.span(),
            Self::IndexingOperator(inner) => inner.span(),
            Self::LetAndOperator(inner) => inner.span(),
            Self::LetOperator(inner) => inner.span(),
            Self::MatchOperator(inner) => inner.span(),
            Self::PrefixOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedPatternChildren<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ParenthesizedPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PolymorphicTypeChildren<'tree> {
    AbstractType(::std::boxed::Box<AbstractType<'tree>>),
    TypeVariable(::std::boxed::Box<TypeVariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PolymorphicTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_type" => Ok(Self::AbstractType(::std::boxed::Box::new(
                <AbstractType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_variable" => Ok(Self::TypeVariable(::std::boxed::Box::new(
                <TypeVariable as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PolymorphicTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractType(inner) => inner.span(),
            Self::TypeVariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PolymorphicVariantTypeChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    Tag(::std::boxed::Box<Tag<'tree>>),
    TagSpecification(::std::boxed::Box<TagSpecification<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PolymorphicVariantTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "tag" => Ok(Self::Tag(::std::boxed::Box::new(
                <Tag as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_specification" => Ok(Self::TagSpecification(::std::boxed::Box::new(
                <TagSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PolymorphicVariantTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
            Self::TagSpecification(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QuotedExtensionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    QuotedStringContent(::std::boxed::Box<QuotedStringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedExtensionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_string_content" => Ok(Self::QuotedStringContent(::std::boxed::Box::new(
                <QuotedStringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedExtensionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::QuotedStringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QuotedItemExtensionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    QuotedStringContent(::std::boxed::Box<QuotedStringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedItemExtensionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_string_content" => Ok(Self::QuotedStringContent(::std::boxed::Box::new(
                <QuotedStringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedItemExtensionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::QuotedStringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QuotedStringContentChildren<'tree> {
    ConversionSpecification(::std::boxed::Box<ConversionSpecification<'tree>>),
    PrettyPrintingIndication(::std::boxed::Box<PrettyPrintingIndication<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedStringContentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "conversion_specification" => {
                Ok(Self::ConversionSpecification(::std::boxed::Box::new(
                    <ConversionSpecification as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "pretty_printing_indication" => {
                Ok(Self::PrettyPrintingIndication(::std::boxed::Box::new(
                    <PrettyPrintingIndication as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedStringContentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConversionSpecification(inner) => inner.span(),
            Self::PrettyPrintingIndication(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SequenceExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SequenceExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::AttributeId(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SetExpressionChildren<'tree> {
    ArrayGetExpression(::std::boxed::Box<ArrayGetExpression<'tree>>),
    BigarrayGetExpression(::std::boxed::Box<BigarrayGetExpression<'tree>>),
    FieldGetExpression(::std::boxed::Box<FieldGetExpression<'tree>>),
    InstanceVariableName(::std::boxed::Box<InstanceVariableName<'tree>>),
    StringGetExpression(::std::boxed::Box<StringGetExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_get_expression" => Ok(Self::ArrayGetExpression(::std::boxed::Box::new(
                <ArrayGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bigarray_get_expression" => Ok(Self::BigarrayGetExpression(::std::boxed::Box::new(
                <BigarrayGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_get_expression" => Ok(Self::FieldGetExpression(::std::boxed::Box::new(
                <FieldGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_variable_name" => Ok(Self::InstanceVariableName(::std::boxed::Box::new(
                <InstanceVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_get_expression" => Ok(Self::StringGetExpression(::std::boxed::Box::new(
                <StringGetExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SetExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayGetExpression(inner) => inner.span(),
            Self::BigarrayGetExpression(inner) => inner.span(),
            Self::FieldGetExpression(inner) => inner.span(),
            Self::InstanceVariableName(inner) => inner.span(),
            Self::StringGetExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringContentChildren<'tree> {
    ConversionSpecification(::std::boxed::Box<ConversionSpecification<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    PrettyPrintingIndication(::std::boxed::Box<PrettyPrintingIndication<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringContentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "conversion_specification" => {
                Ok(Self::ConversionSpecification(::std::boxed::Box::new(
                    <ConversionSpecification as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pretty_printing_indication" => {
                Ok(Self::PrettyPrintingIndication(::std::boxed::Box::new(
                    <PrettyPrintingIndication as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringContentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConversionSpecification(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::PrettyPrintingIndication(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StructureChildren<'tree> {
    StructureItem(::std::boxed::Box<StructureItem<'tree>>),
    ExpressionItem(::std::boxed::Box<ExpressionItem<'tree>>),
    ToplevelDirective(::std::boxed::Box<ToplevelDirective<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructureChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_item" => Ok(Self::ExpressionItem(::std::boxed::Box::new(
                <ExpressionItem as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "toplevel_directive" => Ok(Self::ToplevelDirective(::std::boxed::Box::new(
                <ToplevelDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <StructureItem as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::StructureItem(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for StructureChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::StructureItem(inner) => inner.span(),
            Self::ExpressionItem(inner) => inner.span(),
            Self::ToplevelDirective(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TagPatternPattern<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for TagPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TagSpecificationChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    Tag(::std::boxed::Box<Tag<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagSpecificationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "tag" => Ok(Self::Tag(::std::boxed::Box::new(
                <Tag as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TagSpecificationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ToplevelDirectiveChildren<'tree> {
    Constant(::std::boxed::Box<Constant<'tree>>),
    Directive(::std::boxed::Box<Directive<'tree>>),
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
    ValuePath(::std::boxed::Box<ValuePath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ToplevelDirectiveChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "directive" => Ok(Self::Directive(::std::boxed::Box::new(
                <Directive as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_path" => Ok(Self::ValuePath(::std::boxed::Box::new(
                <ValuePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Constant as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Constant(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ToplevelDirectiveChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constant(inner) => inner.span(),
            Self::Directive(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
            Self::ValuePath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TryExpressionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    MatchCase(::std::boxed::Box<MatchCase<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_case" => Ok(Self::MatchCase(::std::boxed::Box::new(
                <MatchCase as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TryExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::MatchCase(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TupleExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LabeledTupleElement(::std::boxed::Box<LabeledTupleElement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_tuple_element" => Ok(Self::LabeledTupleElement(::std::boxed::Box::new(
                <LabeledTupleElement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TupleExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LabeledTupleElement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TuplePatternChildren<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    LabeledTupleElementBindingPattern(::std::boxed::Box<LabeledTupleElementBindingPattern<'tree>>),
    LabeledTupleElementPattern(::std::boxed::Box<LabeledTupleElementPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TuplePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_tuple_element_binding_pattern" => Ok(Self::LabeledTupleElementBindingPattern(
                ::std::boxed::Box::new(
                    <LabeledTupleElementBindingPattern as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "labeled_tuple_element_pattern" => {
                Ok(Self::LabeledTupleElementPattern(::std::boxed::Box::new(
                    <LabeledTupleElementPattern as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) =
                    <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for TuplePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::LabeledTupleElementBindingPattern(inner) => inner.span(),
            Self::LabeledTupleElementPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TupleTypeChildren<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    LabeledTupleElementType(::std::boxed::Box<LabeledTupleElementType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "labeled_tuple_element_type" => {
                Ok(Self::LabeledTupleElementType(::std::boxed::Box::new(
                    <LabeledTupleElementType as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for TupleTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::LabeledTupleElementType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeBindingBody<'tree> {
    DotDot(::treesitter_types::Span),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
    VariantDeclaration(::std::boxed::Box<VariantDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeBindingBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ".." => Ok(Self::DotDot(::treesitter_types::Span::from(node))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variant_declaration" => Ok(Self::VariantDeclaration(::std::boxed::Box::new(
                <VariantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeBindingBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DotDot(span) => *span,
            Self::RecordDeclaration(inner) => inner.span(),
            Self::VariantDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeBindingName<'tree> {
    TypeConstructor(::std::boxed::Box<TypeConstructor<'tree>>),
    TypeConstructorPath(::std::boxed::Box<TypeConstructorPath<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeBindingName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_constructor" => Ok(Self::TypeConstructor(::std::boxed::Box::new(
                <TypeConstructor as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_constructor_path" => Ok(Self::TypeConstructorPath(::std::boxed::Box::new(
                <TypeConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeBindingName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TypeConstructor(inner) => inner.span(),
            Self::TypeConstructorPath(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeBindingChildren<'tree> {
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    TypeConstraint(::std::boxed::Box<TypeConstraint<'tree>>),
    TypeVariable(::std::boxed::Box<TypeVariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeBindingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_constraint" => Ok(Self::TypeConstraint(::std::boxed::Box::new(
                <TypeConstraint as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_variable" => Ok(Self::TypeVariable(::std::boxed::Box::new(
                <TypeVariable as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeBindingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ItemAttribute(inner) => inner.span(),
            Self::TypeConstraint(inner) => inner.span(),
            Self::TypeVariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeConstructorPathChildren<'tree> {
    ExtendedModulePath(::std::boxed::Box<ExtendedModulePath<'tree>>),
    TypeConstructor(::std::boxed::Box<TypeConstructor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeConstructorPathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "extended_module_path" => Ok(Self::ExtendedModulePath(::std::boxed::Box::new(
                <ExtendedModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_constructor" => Ok(Self::TypeConstructor(::std::boxed::Box::new(
                <TypeConstructor as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeConstructorPathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExtendedModulePath(inner) => inner.span(),
            Self::TypeConstructor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDefinitionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    TypeBinding(::std::boxed::Box<TypeBinding<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_binding" => Ok(Self::TypeBinding(::std::boxed::Box::new(
                <TypeBinding as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::TypeBinding(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypedPatternPattern<'tree> {
    BindingPattern(::std::boxed::Box<BindingPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::BindingPattern(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for TypedPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValueDefinitionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    LetAndOperator(::std::boxed::Box<LetAndOperator<'tree>>),
    LetBinding(::std::boxed::Box<LetBinding<'tree>>),
    LetOperator(::std::boxed::Box<LetOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValueDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_and_operator" => Ok(Self::LetAndOperator(::std::boxed::Box::new(
                <LetAndOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_binding" => Ok(Self::LetBinding(::std::boxed::Box::new(
                <LetBinding as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_operator" => Ok(Self::LetOperator(::std::boxed::Box::new(
                <LetOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValueDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::LetAndOperator(inner) => inner.span(),
            Self::LetBinding(inner) => inner.span(),
            Self::LetOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValuePathChildren<'tree> {
    ModulePath(::std::boxed::Box<ModulePath<'tree>>),
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    ValueName(::std::boxed::Box<ValueName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValuePathChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "module_path" => Ok(Self::ModulePath(::std::boxed::Box::new(
                <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_name" => Ok(Self::ValueName(::std::boxed::Box::new(
                <ValueName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValuePathChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ModulePath(inner) => inner.span(),
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ValueName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValueSpecificationChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    ItemAttribute(::std::boxed::Box<ItemAttribute<'tree>>),
    ParenthesizedOperator(::std::boxed::Box<ParenthesizedOperator<'tree>>),
    ValueName(::std::boxed::Box<ValueName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValueSpecificationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "item_attribute" => Ok(Self::ItemAttribute(::std::boxed::Box::new(
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_operator" => Ok(Self::ParenthesizedOperator(::std::boxed::Box::new(
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_name" => Ok(Self::ValueName(::std::boxed::Box::new(
                <ValueName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValueSpecificationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ValueName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WhileExpressionChildren<'tree> {
    AttributeId(::std::boxed::Box<AttributeId<'tree>>),
    DoClause(::std::boxed::Box<DoClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_id" => Ok(Self::AttributeId(::std::boxed::Box::new(
                <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_clause" => Ok(Self::DoClause(::std::boxed::Box::new(
                <DoClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for WhileExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeId(inner) => inner.span(),
            Self::DoClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    BindingPattern(BindingPattern<'tree>),
    ClassExpression(ClassExpression<'tree>),
    ClassField(ClassField<'tree>),
    ClassFieldSpecification(ClassFieldSpecification<'tree>),
    ClassType(ClassType<'tree>),
    Constant(Constant<'tree>),
    EffectPatternType(EffectPatternType<'tree>),
    Expression(Expression<'tree>),
    InfixOperator(InfixOperator<'tree>),
    ModuleExpression(ModuleExpression<'tree>),
    ModuleType(ModuleType<'tree>),
    ParameterType(ParameterType<'tree>),
    Pattern(Pattern<'tree>),
    PolymorphicTypeType(PolymorphicTypeType<'tree>),
    SequenceExpressionType(SequenceExpressionType<'tree>),
    SignatureItem(SignatureItem<'tree>),
    SignedConstant(SignedConstant<'tree>),
    SimpleBindingPattern(SimpleBindingPattern<'tree>),
    SimpleClassExpression(SimpleClassExpression<'tree>),
    SimpleClassType(SimpleClassType<'tree>),
    SimpleExpression(SimpleExpression<'tree>),
    SimpleModuleExpression(SimpleModuleExpression<'tree>),
    SimplePattern(SimplePattern<'tree>),
    SimpleType(SimpleType<'tree>),
    StructureItem(StructureItem<'tree>),
    Type(Type<'tree>),
    AbstractType(AbstractType<'tree>),
    AddOperator(AddOperator<'tree>),
    AliasPattern(AliasPattern<'tree>),
    AliasedType(AliasedType<'tree>),
    ApplicationExpression(ApplicationExpression<'tree>),
    ArrayBindingPattern(ArrayBindingPattern<'tree>),
    ArrayExpression(ArrayExpression<'tree>),
    ArrayGetExpression(ArrayGetExpression<'tree>),
    ArrayPattern(ArrayPattern<'tree>),
    AssertExpression(AssertExpression<'tree>),
    Attribute(Attribute<'tree>),
    AttributeId(AttributeId<'tree>),
    AttributePayload(AttributePayload<'tree>),
    BigarrayGetExpression(BigarrayGetExpression<'tree>),
    Boolean(Boolean<'tree>),
    Character(Character<'tree>),
    CharacterContent(CharacterContent<'tree>),
    ClassApplication(ClassApplication<'tree>),
    ClassBinding(ClassBinding<'tree>),
    ClassBodyType(ClassBodyType<'tree>),
    ClassDefinition(ClassDefinition<'tree>),
    ClassFunction(ClassFunction<'tree>),
    ClassFunctionType(ClassFunctionType<'tree>),
    ClassInitializer(ClassInitializer<'tree>),
    ClassPath(ClassPath<'tree>),
    ClassTypeBinding(ClassTypeBinding<'tree>),
    ClassTypeDefinition(ClassTypeDefinition<'tree>),
    ClassTypePath(ClassTypePath<'tree>),
    CoercionExpression(CoercionExpression<'tree>),
    CompilationUnit(CompilationUnit<'tree>),
    ConsExpression(ConsExpression<'tree>),
    ConsPattern(ConsPattern<'tree>),
    ConstrainModule(ConstrainModule<'tree>),
    ConstrainModuleType(ConstrainModuleType<'tree>),
    ConstrainType(ConstrainType<'tree>),
    ConstructedType(ConstructedType<'tree>),
    ConstructorDeclaration(ConstructorDeclaration<'tree>),
    ConstructorPath(ConstructorPath<'tree>),
    ConstructorPattern(ConstructorPattern<'tree>),
    Directive(Directive<'tree>),
    DoClause(DoClause<'tree>),
    EffectPattern(EffectPattern<'tree>),
    ElseClause(ElseClause<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    ExceptionDefinition(ExceptionDefinition<'tree>),
    ExceptionPattern(ExceptionPattern<'tree>),
    ExpressionItem(ExpressionItem<'tree>),
    ExtendedModulePath(ExtendedModulePath<'tree>),
    Extension(Extension<'tree>),
    External(External<'tree>),
    FieldDeclaration(FieldDeclaration<'tree>),
    FieldExpression(FieldExpression<'tree>),
    FieldGetExpression(FieldGetExpression<'tree>),
    FieldPath(FieldPath<'tree>),
    FieldPattern(FieldPattern<'tree>),
    FloatingAttribute(FloatingAttribute<'tree>),
    ForExpression(ForExpression<'tree>),
    FunExpression(FunExpression<'tree>),
    FunctionExpression(FunctionExpression<'tree>),
    FunctionType(FunctionType<'tree>),
    Functor(Functor<'tree>),
    FunctorType(FunctorType<'tree>),
    Guard(Guard<'tree>),
    HashExpression(HashExpression<'tree>),
    HashType(HashType<'tree>),
    IfExpression(IfExpression<'tree>),
    IncludeModule(IncludeModule<'tree>),
    IncludeModuleType(IncludeModuleType<'tree>),
    IndexingOperatorPath(IndexingOperatorPath<'tree>),
    InfixExpression(InfixExpression<'tree>),
    InheritanceDefinition(InheritanceDefinition<'tree>),
    InheritanceSpecification(InheritanceSpecification<'tree>),
    InstanceVariableDefinition(InstanceVariableDefinition<'tree>),
    InstanceVariableExpression(InstanceVariableExpression<'tree>),
    InstanceVariableSpecification(InstanceVariableSpecification<'tree>),
    InstantiatedClass(InstantiatedClass<'tree>),
    InstantiatedClassType(InstantiatedClassType<'tree>),
    ItemAttribute(ItemAttribute<'tree>),
    ItemExtension(ItemExtension<'tree>),
    LabeledArgument(LabeledArgument<'tree>),
    LabeledArgumentType(LabeledArgumentType<'tree>),
    LabeledTupleElement(LabeledTupleElement<'tree>),
    LabeledTupleElementBindingPattern(LabeledTupleElementBindingPattern<'tree>),
    LabeledTupleElementPattern(LabeledTupleElementPattern<'tree>),
    LabeledTupleElementType(LabeledTupleElementType<'tree>),
    LazyExpression(LazyExpression<'tree>),
    LazyPattern(LazyPattern<'tree>),
    LetBinding(LetBinding<'tree>),
    LetClassExpression(LetClassExpression<'tree>),
    LetExceptionExpression(LetExceptionExpression<'tree>),
    LetExpression(LetExpression<'tree>),
    LetModuleExpression(LetModuleExpression<'tree>),
    LetOpenClassExpression(LetOpenClassExpression<'tree>),
    LetOpenClassType(LetOpenClassType<'tree>),
    LetOpenExpression(LetOpenExpression<'tree>),
    ListBindingPattern(ListBindingPattern<'tree>),
    ListExpression(ListExpression<'tree>),
    ListPattern(ListPattern<'tree>),
    LocalOpenExpression(LocalOpenExpression<'tree>),
    LocalOpenPattern(LocalOpenPattern<'tree>),
    LocalOpenType(LocalOpenType<'tree>),
    MatchCase(MatchCase<'tree>),
    MatchExpression(MatchExpression<'tree>),
    MethodDefinition(MethodDefinition<'tree>),
    MethodInvocation(MethodInvocation<'tree>),
    MethodSpecification(MethodSpecification<'tree>),
    MethodType(MethodType<'tree>),
    ModuleApplication(ModuleApplication<'tree>),
    ModuleBinding(ModuleBinding<'tree>),
    ModuleDefinition(ModuleDefinition<'tree>),
    ModuleParameter(ModuleParameter<'tree>),
    ModulePath(ModulePath<'tree>),
    ModuleTypeConstraint(ModuleTypeConstraint<'tree>),
    ModuleTypeDefinition(ModuleTypeDefinition<'tree>),
    ModuleTypeOf(ModuleTypeOf<'tree>),
    ModuleTypePath(ModuleTypePath<'tree>),
    NewExpression(NewExpression<'tree>),
    Number(Number<'tree>),
    ObjectCopyExpression(ObjectCopyExpression<'tree>),
    ObjectExpression(ObjectExpression<'tree>),
    ObjectType(ObjectType<'tree>),
    OpenModule(OpenModule<'tree>),
    OrPattern(OrPattern<'tree>),
    PackageExpression(PackageExpression<'tree>),
    PackagePattern(PackagePattern<'tree>),
    PackageType(PackageType<'tree>),
    PackedModule(PackedModule<'tree>),
    Parameter(Parameter<'tree>),
    ParenthesizedClassExpression(ParenthesizedClassExpression<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    ParenthesizedModuleExpression(ParenthesizedModuleExpression<'tree>),
    ParenthesizedModuleType(ParenthesizedModuleType<'tree>),
    ParenthesizedOperator(ParenthesizedOperator<'tree>),
    ParenthesizedPattern(ParenthesizedPattern<'tree>),
    ParenthesizedType(ParenthesizedType<'tree>),
    PolymorphicType(PolymorphicType<'tree>),
    PolymorphicVariantPattern(PolymorphicVariantPattern<'tree>),
    PolymorphicVariantType(PolymorphicVariantType<'tree>),
    PrefixExpression(PrefixExpression<'tree>),
    QuotedExtension(QuotedExtension<'tree>),
    QuotedItemExtension(QuotedItemExtension<'tree>),
    QuotedString(QuotedString<'tree>),
    QuotedStringContent(QuotedStringContent<'tree>),
    RangePattern(RangePattern<'tree>),
    RecordBindingPattern(RecordBindingPattern<'tree>),
    RecordDeclaration(RecordDeclaration<'tree>),
    RecordExpression(RecordExpression<'tree>),
    RecordPattern(RecordPattern<'tree>),
    RefutationCase(RefutationCase<'tree>),
    SequenceExpression(SequenceExpression<'tree>),
    SetExpression(SetExpression<'tree>),
    SignExpression(SignExpression<'tree>),
    SignOperator(SignOperator<'tree>),
    Signature(Signature<'tree>),
    SignedNumber(SignedNumber<'tree>),
    String(String<'tree>),
    StringContent(StringContent<'tree>),
    StringGetExpression(StringGetExpression<'tree>),
    Structure(Structure<'tree>),
    Tag(Tag<'tree>),
    TagPattern(TagPattern<'tree>),
    TagSpecification(TagSpecification<'tree>),
    ThenClause(ThenClause<'tree>),
    ToplevelDirective(ToplevelDirective<'tree>),
    TryExpression(TryExpression<'tree>),
    TupleExpression(TupleExpression<'tree>),
    TuplePattern(TuplePattern<'tree>),
    TupleType(TupleType<'tree>),
    TypeBinding(TypeBinding<'tree>),
    TypeConstraint(TypeConstraint<'tree>),
    TypeConstructorPath(TypeConstructorPath<'tree>),
    TypeDefinition(TypeDefinition<'tree>),
    TypeParameterConstraint(TypeParameterConstraint<'tree>),
    TypeVariable(TypeVariable<'tree>),
    TypedClassExpression(TypedClassExpression<'tree>),
    TypedExpression(TypedExpression<'tree>),
    TypedModuleExpression(TypedModuleExpression<'tree>),
    TypedPattern(TypedPattern<'tree>),
    Unit(Unit<'tree>),
    ValueDefinition(ValueDefinition<'tree>),
    ValuePath(ValuePath<'tree>),
    ValueSpecification(ValueSpecification<'tree>),
    VariantDeclaration(VariantDeclaration<'tree>),
    WhileExpression(WhileExpression<'tree>),
    AndOperator(AndOperator<'tree>),
    AssignOperator(AssignOperator<'tree>),
    ClassName(ClassName<'tree>),
    ClassTypeName(ClassTypeName<'tree>),
    Comment(Comment<'tree>),
    ConcatOperator(ConcatOperator<'tree>),
    ConstructorName(ConstructorName<'tree>),
    ConversionSpecification(ConversionSpecification<'tree>),
    FieldName(FieldName<'tree>),
    HashOperator(HashOperator<'tree>),
    IndexingOperator(IndexingOperator<'tree>),
    InstanceVariableName(InstanceVariableName<'tree>),
    LabelName(LabelName<'tree>),
    LetAndOperator(LetAndOperator<'tree>),
    LetOperator(LetOperator<'tree>),
    LineNumberDirective(LineNumberDirective<'tree>),
    MatchOperator(MatchOperator<'tree>),
    MethodName(MethodName<'tree>),
    ModuleName(ModuleName<'tree>),
    ModuleTypeName(ModuleTypeName<'tree>),
    MultOperator(MultOperator<'tree>),
    OcamlyaccValue(OcamlyaccValue<'tree>),
    OrOperator(OrOperator<'tree>),
    PowOperator(PowOperator<'tree>),
    PrefixOperator(PrefixOperator<'tree>),
    PrettyPrintingIndication(PrettyPrintingIndication<'tree>),
    RelOperator(RelOperator<'tree>),
    Shebang(Shebang<'tree>),
    TypeConstructor(TypeConstructor<'tree>),
    ValueName(ValueName<'tree>),
    ValuePattern(ValuePattern<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_binding_pattern" => {
                <BindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BindingPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "_class_expression" => {
                <ClassExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "_class_field" => <ClassField as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ClassField)
                .unwrap_or(Self::Unknown(node)),
            "_class_field_specification" => {
                <ClassFieldSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassFieldSpecification)
                    .unwrap_or(Self::Unknown(node))
            }
            "_class_type" => <ClassType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ClassType)
                .unwrap_or(Self::Unknown(node)),
            "_constant" => <Constant as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Constant)
                .unwrap_or(Self::Unknown(node)),
            "_effect_pattern" => {
                <EffectPatternType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EffectPatternType)
                    .unwrap_or(Self::Unknown(node))
            }
            "_expression" => <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expression)
                .unwrap_or(Self::Unknown(node)),
            "_infix_operator" => {
                <InfixOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InfixOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "_module_expression" => {
                <ModuleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "_module_type" => <ModuleType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ModuleType)
                .unwrap_or(Self::Unknown(node)),
            "_parameter" => <ParameterType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ParameterType)
                .unwrap_or(Self::Unknown(node)),
            "_pattern" => <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pattern)
                .unwrap_or(Self::Unknown(node)),
            "_polymorphic_type" => {
                <PolymorphicTypeType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PolymorphicTypeType)
                    .unwrap_or(Self::Unknown(node))
            }
            "_sequence_expression" => {
                <SequenceExpressionType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SequenceExpressionType)
                    .unwrap_or(Self::Unknown(node))
            }
            "_signature_item" => {
                <SignatureItem as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SignatureItem)
                    .unwrap_or(Self::Unknown(node))
            }
            "_signed_constant" => {
                <SignedConstant as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SignedConstant)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_binding_pattern" => {
                <SimpleBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleBindingPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_class_expression" => {
                <SimpleClassExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleClassExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_class_type" => {
                <SimpleClassType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleClassType)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_expression" => {
                <SimpleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_module_expression" => {
                <SimpleModuleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleModuleExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_pattern" => {
                <SimplePattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimplePattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_type" => <SimpleType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SimpleType)
                .unwrap_or(Self::Unknown(node)),
            "_structure_item" => {
                <StructureItem as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StructureItem)
                    .unwrap_or(Self::Unknown(node))
            }
            "_type" => <Type as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Type)
                .unwrap_or(Self::Unknown(node)),
            "abstract_type" => <AbstractType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AbstractType)
                .unwrap_or(Self::Unknown(node)),
            "add_operator" => <AddOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AddOperator)
                .unwrap_or(Self::Unknown(node)),
            "alias_pattern" => <AliasPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AliasPattern)
                .unwrap_or(Self::Unknown(node)),
            "aliased_type" => <AliasedType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AliasedType)
                .unwrap_or(Self::Unknown(node)),
            "application_expression" => {
                <ApplicationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ApplicationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_binding_pattern" => {
                <ArrayBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayBindingPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_expression" => {
                <ArrayExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_get_expression" => {
                <ArrayGetExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayGetExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_pattern" => <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ArrayPattern)
                .unwrap_or(Self::Unknown(node)),
            "assert_expression" => {
                <AssertExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssertExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute" => <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Attribute)
                .unwrap_or(Self::Unknown(node)),
            "attribute_id" => <AttributeId as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AttributeId)
                .unwrap_or(Self::Unknown(node)),
            "attribute_payload" => {
                <AttributePayload as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributePayload)
                    .unwrap_or(Self::Unknown(node))
            }
            "bigarray_get_expression" => {
                <BigarrayGetExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BigarrayGetExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "boolean" => <Boolean as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Boolean)
                .unwrap_or(Self::Unknown(node)),
            "character" => <Character as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Character)
                .unwrap_or(Self::Unknown(node)),
            "character_content" => {
                <CharacterContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CharacterContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_application" => {
                <ClassApplication as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassApplication)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_binding" => <ClassBinding as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ClassBinding)
                .unwrap_or(Self::Unknown(node)),
            "class_body_type" => {
                <ClassBodyType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassBodyType)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_definition" => {
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_function" => {
                <ClassFunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassFunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_function_type" => {
                <ClassFunctionType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassFunctionType)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_initializer" => {
                <ClassInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassInitializer)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_path" => <ClassPath as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ClassPath)
                .unwrap_or(Self::Unknown(node)),
            "class_type_binding" => {
                <ClassTypeBinding as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassTypeBinding)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_type_definition" => {
                <ClassTypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassTypeDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_type_path" => {
                <ClassTypePath as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassTypePath)
                    .unwrap_or(Self::Unknown(node))
            }
            "coercion_expression" => {
                <CoercionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CoercionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "compilation_unit" => {
                <CompilationUnit as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompilationUnit)
                    .unwrap_or(Self::Unknown(node))
            }
            "cons_expression" => {
                <ConsExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConsExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "cons_pattern" => <ConsPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ConsPattern)
                .unwrap_or(Self::Unknown(node)),
            "constrain_module" => {
                <ConstrainModule as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstrainModule)
                    .unwrap_or(Self::Unknown(node))
            }
            "constrain_module_type" => {
                <ConstrainModuleType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstrainModuleType)
                    .unwrap_or(Self::Unknown(node))
            }
            "constrain_type" => {
                <ConstrainType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstrainType)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructed_type" => {
                <ConstructedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_declaration" => {
                <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructorDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_path" => {
                <ConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructorPath)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_pattern" => {
                <ConstructorPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructorPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "directive" => <Directive as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Directive)
                .unwrap_or(Self::Unknown(node)),
            "do_clause" => <DoClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoClause)
                .unwrap_or(Self::Unknown(node)),
            "effect_pattern" => {
                <EffectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EffectPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "else_clause" => <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElseClause)
                .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "exception_definition" => {
                <ExceptionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExceptionDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "exception_pattern" => {
                <ExceptionPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExceptionPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_item" => {
                <ExpressionItem as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionItem)
                    .unwrap_or(Self::Unknown(node))
            }
            "extended_module_path" => {
                <ExtendedModulePath as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExtendedModulePath)
                    .unwrap_or(Self::Unknown(node))
            }
            "extension" => <Extension as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Extension)
                .unwrap_or(Self::Unknown(node)),
            "external" => <External as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::External)
                .unwrap_or(Self::Unknown(node)),
            "field_declaration" => {
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_expression" => {
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_get_expression" => {
                <FieldGetExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldGetExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_path" => <FieldPath as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FieldPath)
                .unwrap_or(Self::Unknown(node)),
            "field_pattern" => <FieldPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FieldPattern)
                .unwrap_or(Self::Unknown(node)),
            "floating_attribute" => {
                <FloatingAttribute as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FloatingAttribute)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_expression" => {
                <ForExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "fun_expression" => {
                <FunExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_expression" => {
                <FunctionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_type" => <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FunctionType)
                .unwrap_or(Self::Unknown(node)),
            "functor" => <Functor as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Functor)
                .unwrap_or(Self::Unknown(node)),
            "functor_type" => <FunctorType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FunctorType)
                .unwrap_or(Self::Unknown(node)),
            "guard" => <Guard as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Guard)
                .unwrap_or(Self::Unknown(node)),
            "hash_expression" => {
                <HashExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::HashExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "hash_type" => <HashType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HashType)
                .unwrap_or(Self::Unknown(node)),
            "if_expression" => <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfExpression)
                .unwrap_or(Self::Unknown(node)),
            "include_module" => {
                <IncludeModule as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IncludeModule)
                    .unwrap_or(Self::Unknown(node))
            }
            "include_module_type" => {
                <IncludeModuleType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IncludeModuleType)
                    .unwrap_or(Self::Unknown(node))
            }
            "indexing_operator_path" => {
                <IndexingOperatorPath as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IndexingOperatorPath)
                    .unwrap_or(Self::Unknown(node))
            }
            "infix_expression" => {
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InfixExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "inheritance_definition" => {
                <InheritanceDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InheritanceDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "inheritance_specification" => {
                <InheritanceSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InheritanceSpecification)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance_variable_definition" => {
                <InstanceVariableDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InstanceVariableDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance_variable_expression" => {
                <InstanceVariableExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InstanceVariableExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance_variable_specification" => {
                <InstanceVariableSpecification as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::InstanceVariableSpecification)
                .unwrap_or(Self::Unknown(node))
            }
            "instantiated_class" => {
                <InstantiatedClass as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InstantiatedClass)
                    .unwrap_or(Self::Unknown(node))
            }
            "instantiated_class_type" => {
                <InstantiatedClassType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InstantiatedClassType)
                    .unwrap_or(Self::Unknown(node))
            }
            "item_attribute" => {
                <ItemAttribute as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ItemAttribute)
                    .unwrap_or(Self::Unknown(node))
            }
            "item_extension" => {
                <ItemExtension as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ItemExtension)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_argument" => {
                <LabeledArgument as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledArgument)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_argument_type" => {
                <LabeledArgumentType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledArgumentType)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_tuple_element" => {
                <LabeledTupleElement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledTupleElement)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_tuple_element_binding_pattern" => {
                <LabeledTupleElementBindingPattern as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::LabeledTupleElementBindingPattern)
                .unwrap_or(Self::Unknown(node))
            }
            "labeled_tuple_element_pattern" => {
                <LabeledTupleElementPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledTupleElementPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_tuple_element_type" => {
                <LabeledTupleElementType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledTupleElementType)
                    .unwrap_or(Self::Unknown(node))
            }
            "lazy_expression" => {
                <LazyExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LazyExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "lazy_pattern" => <LazyPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::LazyPattern)
                .unwrap_or(Self::Unknown(node)),
            "let_binding" => <LetBinding as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::LetBinding)
                .unwrap_or(Self::Unknown(node)),
            "let_class_expression" => {
                <LetClassExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetClassExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_exception_expression" => {
                <LetExceptionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetExceptionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_expression" => {
                <LetExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_module_expression" => {
                <LetModuleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetModuleExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_open_class_expression" => {
                <LetOpenClassExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetOpenClassExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_open_class_type" => {
                <LetOpenClassType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetOpenClassType)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_open_expression" => {
                <LetOpenExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetOpenExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_binding_pattern" => {
                <ListBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListBindingPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_expression" => {
                <ListExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_pattern" => <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ListPattern)
                .unwrap_or(Self::Unknown(node)),
            "local_open_expression" => {
                <LocalOpenExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalOpenExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "local_open_pattern" => {
                <LocalOpenPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalOpenPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "local_open_type" => {
                <LocalOpenType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalOpenType)
                    .unwrap_or(Self::Unknown(node))
            }
            "match_case" => <MatchCase as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MatchCase)
                .unwrap_or(Self::Unknown(node)),
            "match_expression" => {
                <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_definition" => {
                <MethodDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_invocation" => {
                <MethodInvocation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodInvocation)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_specification" => {
                <MethodSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodSpecification)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_type" => <MethodType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MethodType)
                .unwrap_or(Self::Unknown(node)),
            "module_application" => {
                <ModuleApplication as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleApplication)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_binding" => {
                <ModuleBinding as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleBinding)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_definition" => {
                <ModuleDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_parameter" => {
                <ModuleParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_path" => <ModulePath as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ModulePath)
                .unwrap_or(Self::Unknown(node)),
            "module_type_constraint" => {
                <ModuleTypeConstraint as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleTypeConstraint)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_type_definition" => {
                <ModuleTypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleTypeDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_type_of" => {
                <ModuleTypeOf as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleTypeOf)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_type_path" => {
                <ModuleTypePath as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleTypePath)
                    .unwrap_or(Self::Unknown(node))
            }
            "new_expression" => {
                <NewExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NewExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "number" => <Number as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Number)
                .unwrap_or(Self::Unknown(node)),
            "object_copy_expression" => {
                <ObjectCopyExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ObjectCopyExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "object_expression" => {
                <ObjectExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ObjectExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "object_type" => <ObjectType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ObjectType)
                .unwrap_or(Self::Unknown(node)),
            "open_module" => <OpenModule as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OpenModule)
                .unwrap_or(Self::Unknown(node)),
            "or_pattern" => <OrPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OrPattern)
                .unwrap_or(Self::Unknown(node)),
            "package_expression" => {
                <PackageExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PackageExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "package_pattern" => {
                <PackagePattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PackagePattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "package_type" => <PackageType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PackageType)
                .unwrap_or(Self::Unknown(node)),
            "packed_module" => <PackedModule as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PackedModule)
                .unwrap_or(Self::Unknown(node)),
            "parameter" => <Parameter as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Parameter)
                .unwrap_or(Self::Unknown(node)),
            "parenthesized_class_expression" => {
                <ParenthesizedClassExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedClassExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_module_expression" => {
                <ParenthesizedModuleExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ParenthesizedModuleExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_module_type" => {
                <ParenthesizedModuleType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedModuleType)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_operator" => {
                <ParenthesizedOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_pattern" => {
                <ParenthesizedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_type" => {
                <ParenthesizedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "polymorphic_type" => {
                <PolymorphicType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PolymorphicType)
                    .unwrap_or(Self::Unknown(node))
            }
            "polymorphic_variant_pattern" => {
                <PolymorphicVariantPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PolymorphicVariantPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "polymorphic_variant_type" => {
                <PolymorphicVariantType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PolymorphicVariantType)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_expression" => {
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_extension" => {
                <QuotedExtension as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedExtension)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_item_extension" => {
                <QuotedItemExtension as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedItemExtension)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_string" => <QuotedString as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::QuotedString)
                .unwrap_or(Self::Unknown(node)),
            "quoted_string_content" => {
                <QuotedStringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedStringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "range_pattern" => <RangePattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RangePattern)
                .unwrap_or(Self::Unknown(node)),
            "record_binding_pattern" => {
                <RecordBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RecordBindingPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "record_declaration" => {
                <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RecordDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "record_expression" => {
                <RecordExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RecordExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "record_pattern" => {
                <RecordPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RecordPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "refutation_case" => {
                <RefutationCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RefutationCase)
                    .unwrap_or(Self::Unknown(node))
            }
            "sequence_expression" => {
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SequenceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "set_expression" => {
                <SetExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SetExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "sign_expression" => {
                <SignExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SignExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "sign_operator" => <SignOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SignOperator)
                .unwrap_or(Self::Unknown(node)),
            "signature" => <Signature as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Signature)
                .unwrap_or(Self::Unknown(node)),
            "signed_number" => <SignedNumber as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SignedNumber)
                .unwrap_or(Self::Unknown(node)),
            "string" => <String as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::String)
                .unwrap_or(Self::Unknown(node)),
            "string_content" => {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_get_expression" => {
                <StringGetExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringGetExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "structure" => <Structure as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Structure)
                .unwrap_or(Self::Unknown(node)),
            "tag" => <Tag as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Tag)
                .unwrap_or(Self::Unknown(node)),
            "tag_pattern" => <TagPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TagPattern)
                .unwrap_or(Self::Unknown(node)),
            "tag_specification" => {
                <TagSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TagSpecification)
                    .unwrap_or(Self::Unknown(node))
            }
            "then_clause" => <ThenClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ThenClause)
                .unwrap_or(Self::Unknown(node)),
            "toplevel_directive" => {
                <ToplevelDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ToplevelDirective)
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
            "type_binding" => <TypeBinding as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TypeBinding)
                .unwrap_or(Self::Unknown(node)),
            "type_constraint" => {
                <TypeConstraint as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeConstraint)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_constructor_path" => {
                <TypeConstructorPath as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeConstructorPath)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_definition" => {
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter_constraint" => {
                <TypeParameterConstraint as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameterConstraint)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_variable" => <TypeVariable as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TypeVariable)
                .unwrap_or(Self::Unknown(node)),
            "typed_class_expression" => {
                <TypedClassExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypedClassExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "typed_expression" => {
                <TypedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "typed_module_expression" => {
                <TypedModuleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypedModuleExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "typed_pattern" => <TypedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TypedPattern)
                .unwrap_or(Self::Unknown(node)),
            "unit" => <Unit as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Unit)
                .unwrap_or(Self::Unknown(node)),
            "value_definition" => {
                <ValueDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ValueDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "value_path" => <ValuePath as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ValuePath)
                .unwrap_or(Self::Unknown(node)),
            "value_specification" => {
                <ValueSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ValueSpecification)
                    .unwrap_or(Self::Unknown(node))
            }
            "variant_declaration" => {
                <VariantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariantDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "while_expression" => {
                <WhileExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "and_operator" => <AndOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AndOperator)
                .unwrap_or(Self::Unknown(node)),
            "assign_operator" => {
                <AssignOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssignOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_name" => <ClassName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ClassName)
                .unwrap_or(Self::Unknown(node)),
            "class_type_name" => {
                <ClassTypeName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassTypeName)
                    .unwrap_or(Self::Unknown(node))
            }
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "concat_operator" => {
                <ConcatOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConcatOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_name" => {
                <ConstructorName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructorName)
                    .unwrap_or(Self::Unknown(node))
            }
            "conversion_specification" => {
                <ConversionSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConversionSpecification)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_name" => <FieldName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FieldName)
                .unwrap_or(Self::Unknown(node)),
            "hash_operator" => <HashOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HashOperator)
                .unwrap_or(Self::Unknown(node)),
            "indexing_operator" => {
                <IndexingOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IndexingOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance_variable_name" => {
                <InstanceVariableName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InstanceVariableName)
                    .unwrap_or(Self::Unknown(node))
            }
            "label_name" => <LabelName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::LabelName)
                .unwrap_or(Self::Unknown(node)),
            "let_and_operator" => {
                <LetAndOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetAndOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_operator" => <LetOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::LetOperator)
                .unwrap_or(Self::Unknown(node)),
            "line_number_directive" => {
                <LineNumberDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LineNumberDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "match_operator" => {
                <MatchOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_name" => <MethodName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MethodName)
                .unwrap_or(Self::Unknown(node)),
            "module_name" => <ModuleName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ModuleName)
                .unwrap_or(Self::Unknown(node)),
            "module_type_name" => {
                <ModuleTypeName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleTypeName)
                    .unwrap_or(Self::Unknown(node))
            }
            "mult_operator" => <MultOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MultOperator)
                .unwrap_or(Self::Unknown(node)),
            "ocamlyacc_value" => {
                <OcamlyaccValue as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OcamlyaccValue)
                    .unwrap_or(Self::Unknown(node))
            }
            "or_operator" => <OrOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OrOperator)
                .unwrap_or(Self::Unknown(node)),
            "pow_operator" => <PowOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PowOperator)
                .unwrap_or(Self::Unknown(node)),
            "prefix_operator" => {
                <PrefixOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "pretty_printing_indication" => {
                <PrettyPrintingIndication as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrettyPrintingIndication)
                    .unwrap_or(Self::Unknown(node))
            }
            "rel_operator" => <RelOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RelOperator)
                .unwrap_or(Self::Unknown(node)),
            "shebang" => <Shebang as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Shebang)
                .unwrap_or(Self::Unknown(node)),
            "type_constructor" => {
                <TypeConstructor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeConstructor)
                    .unwrap_or(Self::Unknown(node))
            }
            "value_name" => <ValueName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ValueName)
                .unwrap_or(Self::Unknown(node)),
            "value_pattern" => <ValuePattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ValuePattern)
                .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingPattern(inner) => inner.span(),
            Self::ClassExpression(inner) => inner.span(),
            Self::ClassField(inner) => inner.span(),
            Self::ClassFieldSpecification(inner) => inner.span(),
            Self::ClassType(inner) => inner.span(),
            Self::Constant(inner) => inner.span(),
            Self::EffectPatternType(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::InfixOperator(inner) => inner.span(),
            Self::ModuleExpression(inner) => inner.span(),
            Self::ModuleType(inner) => inner.span(),
            Self::ParameterType(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::PolymorphicTypeType(inner) => inner.span(),
            Self::SequenceExpressionType(inner) => inner.span(),
            Self::SignatureItem(inner) => inner.span(),
            Self::SignedConstant(inner) => inner.span(),
            Self::SimpleBindingPattern(inner) => inner.span(),
            Self::SimpleClassExpression(inner) => inner.span(),
            Self::SimpleClassType(inner) => inner.span(),
            Self::SimpleExpression(inner) => inner.span(),
            Self::SimpleModuleExpression(inner) => inner.span(),
            Self::SimplePattern(inner) => inner.span(),
            Self::SimpleType(inner) => inner.span(),
            Self::StructureItem(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::AbstractType(inner) => inner.span(),
            Self::AddOperator(inner) => inner.span(),
            Self::AliasPattern(inner) => inner.span(),
            Self::AliasedType(inner) => inner.span(),
            Self::ApplicationExpression(inner) => inner.span(),
            Self::ArrayBindingPattern(inner) => inner.span(),
            Self::ArrayExpression(inner) => inner.span(),
            Self::ArrayGetExpression(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::AssertExpression(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::AttributeId(inner) => inner.span(),
            Self::AttributePayload(inner) => inner.span(),
            Self::BigarrayGetExpression(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Character(inner) => inner.span(),
            Self::CharacterContent(inner) => inner.span(),
            Self::ClassApplication(inner) => inner.span(),
            Self::ClassBinding(inner) => inner.span(),
            Self::ClassBodyType(inner) => inner.span(),
            Self::ClassDefinition(inner) => inner.span(),
            Self::ClassFunction(inner) => inner.span(),
            Self::ClassFunctionType(inner) => inner.span(),
            Self::ClassInitializer(inner) => inner.span(),
            Self::ClassPath(inner) => inner.span(),
            Self::ClassTypeBinding(inner) => inner.span(),
            Self::ClassTypeDefinition(inner) => inner.span(),
            Self::ClassTypePath(inner) => inner.span(),
            Self::CoercionExpression(inner) => inner.span(),
            Self::CompilationUnit(inner) => inner.span(),
            Self::ConsExpression(inner) => inner.span(),
            Self::ConsPattern(inner) => inner.span(),
            Self::ConstrainModule(inner) => inner.span(),
            Self::ConstrainModuleType(inner) => inner.span(),
            Self::ConstrainType(inner) => inner.span(),
            Self::ConstructedType(inner) => inner.span(),
            Self::ConstructorDeclaration(inner) => inner.span(),
            Self::ConstructorPath(inner) => inner.span(),
            Self::ConstructorPattern(inner) => inner.span(),
            Self::Directive(inner) => inner.span(),
            Self::DoClause(inner) => inner.span(),
            Self::EffectPattern(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::ExceptionDefinition(inner) => inner.span(),
            Self::ExceptionPattern(inner) => inner.span(),
            Self::ExpressionItem(inner) => inner.span(),
            Self::ExtendedModulePath(inner) => inner.span(),
            Self::Extension(inner) => inner.span(),
            Self::External(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FieldGetExpression(inner) => inner.span(),
            Self::FieldPath(inner) => inner.span(),
            Self::FieldPattern(inner) => inner.span(),
            Self::FloatingAttribute(inner) => inner.span(),
            Self::ForExpression(inner) => inner.span(),
            Self::FunExpression(inner) => inner.span(),
            Self::FunctionExpression(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::Functor(inner) => inner.span(),
            Self::FunctorType(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
            Self::HashExpression(inner) => inner.span(),
            Self::HashType(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
            Self::IncludeModule(inner) => inner.span(),
            Self::IncludeModuleType(inner) => inner.span(),
            Self::IndexingOperatorPath(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InheritanceDefinition(inner) => inner.span(),
            Self::InheritanceSpecification(inner) => inner.span(),
            Self::InstanceVariableDefinition(inner) => inner.span(),
            Self::InstanceVariableExpression(inner) => inner.span(),
            Self::InstanceVariableSpecification(inner) => inner.span(),
            Self::InstantiatedClass(inner) => inner.span(),
            Self::InstantiatedClassType(inner) => inner.span(),
            Self::ItemAttribute(inner) => inner.span(),
            Self::ItemExtension(inner) => inner.span(),
            Self::LabeledArgument(inner) => inner.span(),
            Self::LabeledArgumentType(inner) => inner.span(),
            Self::LabeledTupleElement(inner) => inner.span(),
            Self::LabeledTupleElementBindingPattern(inner) => inner.span(),
            Self::LabeledTupleElementPattern(inner) => inner.span(),
            Self::LabeledTupleElementType(inner) => inner.span(),
            Self::LazyExpression(inner) => inner.span(),
            Self::LazyPattern(inner) => inner.span(),
            Self::LetBinding(inner) => inner.span(),
            Self::LetClassExpression(inner) => inner.span(),
            Self::LetExceptionExpression(inner) => inner.span(),
            Self::LetExpression(inner) => inner.span(),
            Self::LetModuleExpression(inner) => inner.span(),
            Self::LetOpenClassExpression(inner) => inner.span(),
            Self::LetOpenClassType(inner) => inner.span(),
            Self::LetOpenExpression(inner) => inner.span(),
            Self::ListBindingPattern(inner) => inner.span(),
            Self::ListExpression(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::LocalOpenExpression(inner) => inner.span(),
            Self::LocalOpenPattern(inner) => inner.span(),
            Self::LocalOpenType(inner) => inner.span(),
            Self::MatchCase(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::MethodDefinition(inner) => inner.span(),
            Self::MethodInvocation(inner) => inner.span(),
            Self::MethodSpecification(inner) => inner.span(),
            Self::MethodType(inner) => inner.span(),
            Self::ModuleApplication(inner) => inner.span(),
            Self::ModuleBinding(inner) => inner.span(),
            Self::ModuleDefinition(inner) => inner.span(),
            Self::ModuleParameter(inner) => inner.span(),
            Self::ModulePath(inner) => inner.span(),
            Self::ModuleTypeConstraint(inner) => inner.span(),
            Self::ModuleTypeDefinition(inner) => inner.span(),
            Self::ModuleTypeOf(inner) => inner.span(),
            Self::ModuleTypePath(inner) => inner.span(),
            Self::NewExpression(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ObjectCopyExpression(inner) => inner.span(),
            Self::ObjectExpression(inner) => inner.span(),
            Self::ObjectType(inner) => inner.span(),
            Self::OpenModule(inner) => inner.span(),
            Self::OrPattern(inner) => inner.span(),
            Self::PackageExpression(inner) => inner.span(),
            Self::PackagePattern(inner) => inner.span(),
            Self::PackageType(inner) => inner.span(),
            Self::PackedModule(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::ParenthesizedClassExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::ParenthesizedModuleExpression(inner) => inner.span(),
            Self::ParenthesizedModuleType(inner) => inner.span(),
            Self::ParenthesizedOperator(inner) => inner.span(),
            Self::ParenthesizedPattern(inner) => inner.span(),
            Self::ParenthesizedType(inner) => inner.span(),
            Self::PolymorphicType(inner) => inner.span(),
            Self::PolymorphicVariantPattern(inner) => inner.span(),
            Self::PolymorphicVariantType(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuotedExtension(inner) => inner.span(),
            Self::QuotedItemExtension(inner) => inner.span(),
            Self::QuotedString(inner) => inner.span(),
            Self::QuotedStringContent(inner) => inner.span(),
            Self::RangePattern(inner) => inner.span(),
            Self::RecordBindingPattern(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
            Self::RecordExpression(inner) => inner.span(),
            Self::RecordPattern(inner) => inner.span(),
            Self::RefutationCase(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
            Self::SetExpression(inner) => inner.span(),
            Self::SignExpression(inner) => inner.span(),
            Self::SignOperator(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::SignedNumber(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::StringGetExpression(inner) => inner.span(),
            Self::Structure(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
            Self::TagPattern(inner) => inner.span(),
            Self::TagSpecification(inner) => inner.span(),
            Self::ThenClause(inner) => inner.span(),
            Self::ToplevelDirective(inner) => inner.span(),
            Self::TryExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeBinding(inner) => inner.span(),
            Self::TypeConstraint(inner) => inner.span(),
            Self::TypeConstructorPath(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeParameterConstraint(inner) => inner.span(),
            Self::TypeVariable(inner) => inner.span(),
            Self::TypedClassExpression(inner) => inner.span(),
            Self::TypedExpression(inner) => inner.span(),
            Self::TypedModuleExpression(inner) => inner.span(),
            Self::TypedPattern(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::ValueDefinition(inner) => inner.span(),
            Self::ValuePath(inner) => inner.span(),
            Self::ValueSpecification(inner) => inner.span(),
            Self::VariantDeclaration(inner) => inner.span(),
            Self::WhileExpression(inner) => inner.span(),
            Self::AndOperator(inner) => inner.span(),
            Self::AssignOperator(inner) => inner.span(),
            Self::ClassName(inner) => inner.span(),
            Self::ClassTypeName(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::ConcatOperator(inner) => inner.span(),
            Self::ConstructorName(inner) => inner.span(),
            Self::ConversionSpecification(inner) => inner.span(),
            Self::FieldName(inner) => inner.span(),
            Self::HashOperator(inner) => inner.span(),
            Self::IndexingOperator(inner) => inner.span(),
            Self::InstanceVariableName(inner) => inner.span(),
            Self::LabelName(inner) => inner.span(),
            Self::LetAndOperator(inner) => inner.span(),
            Self::LetOperator(inner) => inner.span(),
            Self::LineNumberDirective(inner) => inner.span(),
            Self::MatchOperator(inner) => inner.span(),
            Self::MethodName(inner) => inner.span(),
            Self::ModuleName(inner) => inner.span(),
            Self::ModuleTypeName(inner) => inner.span(),
            Self::MultOperator(inner) => inner.span(),
            Self::OcamlyaccValue(inner) => inner.span(),
            Self::OrOperator(inner) => inner.span(),
            Self::PowOperator(inner) => inner.span(),
            Self::PrefixOperator(inner) => inner.span(),
            Self::PrettyPrintingIndication(inner) => inner.span(),
            Self::RelOperator(inner) => inner.span(),
            Self::Shebang(inner) => inner.span(),
            Self::TypeConstructor(inner) => inner.span(),
            Self::ValueName(inner) => inner.span(),
            Self::ValuePattern(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
