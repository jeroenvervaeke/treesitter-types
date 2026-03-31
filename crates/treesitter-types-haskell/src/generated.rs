#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassDecl<'tree> {
    DataFamily(::std::boxed::Box<DataFamily<'tree>>),
    Decl(::std::boxed::Box<Decl<'tree>>),
    DefaultSignature(::std::boxed::Box<DefaultSignature<'tree>>),
    Fixity(::std::boxed::Box<Fixity<'tree>>),
    TypeFamily(::std::boxed::Box<TypeFamily<'tree>>),
    TypeInstance(::std::boxed::Box<TypeInstance<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDecl<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "data_family" => {
                Ok(
                    Self::DataFamily(
                        ::std::boxed::Box::new(
                            <DataFamily as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "default_signature" => {
                Ok(
                    Self::DefaultSignature(
                        ::std::boxed::Box::new(
                            <DefaultSignature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "fixity" => {
                Ok(
                    Self::Fixity(
                        ::std::boxed::Box::new(
                            <Fixity as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_family" => {
                Ok(
                    Self::TypeFamily(
                        ::std::boxed::Box::new(
                            <TypeFamily as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_instance" => {
                Ok(
                    Self::TypeInstance(
                        ::std::boxed::Box::new(
                            <TypeInstance as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Decl as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Decl(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ClassDecl<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DataFamily(inner) => inner.span(),
            Self::Decl(inner) => inner.span(),
            Self::DefaultSignature(inner) => inner.span(),
            Self::Fixity(inner) => inner.span(),
            Self::TypeFamily(inner) => inner.span(),
            Self::TypeInstance(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint<'tree> {
    Apply(::std::boxed::Box<Apply<'tree>>),
    Infix(::std::boxed::Box<Infix<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixTuple(::std::boxed::Box<PrefixTuple<'tree>>),
    PrefixUnboxedSum(::std::boxed::Box<PrefixUnboxedSum<'tree>>),
    PrefixUnboxedTuple(::std::boxed::Box<PrefixUnboxedTuple<'tree>>),
    Promoted(::std::boxed::Box<Promoted<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Quasiquote(::std::boxed::Box<Quasiquote<'tree>>),
    Splice(::std::boxed::Box<Splice<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnboxedUnit(::std::boxed::Box<UnboxedUnit<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Constraint<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "apply" => {
                Ok(
                    Self::Apply(
                        ::std::boxed::Box::new(
                            <Apply as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "literal" => {
                Ok(
                    Self::Literal(
                        ::std::boxed::Box::new(
                            <Literal as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_tuple" => {
                Ok(
                    Self::PrefixTuple(
                        ::std::boxed::Box::new(
                            <PrefixTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_sum" => {
                Ok(
                    Self::PrefixUnboxedSum(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_tuple" => {
                Ok(
                    Self::PrefixUnboxedTuple(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "promoted" => {
                Ok(
                    Self::Promoted(
                        ::std::boxed::Box::new(
                            <Promoted as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quasiquote" => {
                Ok(
                    Self::Quasiquote(
                        ::std::boxed::Box::new(
                            <Quasiquote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "splice" => {
                Ok(
                    Self::Splice(
                        ::std::boxed::Box::new(
                            <Splice as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tuple" => {
                Ok(
                    Self::Tuple(
                        ::std::boxed::Box::new(
                            <Tuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_unit" => {
                Ok(
                    Self::UnboxedUnit(
                        ::std::boxed::Box::new(
                            <UnboxedUnit as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "wildcard" => {
                Ok(
                    Self::Wildcard(
                        ::std::boxed::Box::new(
                            <Wildcard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Constraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Apply(inner) => inner.span(),
            Self::Infix(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixTuple(inner) => inner.span(),
            Self::PrefixUnboxedSum(inner) => inner.span(),
            Self::PrefixUnboxedTuple(inner) => inner.span(),
            Self::Promoted(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Quasiquote(inner) => inner.span(),
            Self::Splice(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnboxedUnit(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraints<'tree> {
    Constraint(::std::boxed::Box<Constraint<'tree>>),
    Context(::std::boxed::Box<Context<'tree>>),
    Forall(::std::boxed::Box<Forall<'tree>>),
    ImplicitParameter(::std::boxed::Box<ImplicitParameter<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Constraints<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "context" => {
                Ok(
                    Self::Context(
                        ::std::boxed::Box::new(
                            <Context as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "implicit_parameter" => {
                Ok(
                    Self::ImplicitParameter(
                        ::std::boxed::Box::new(
                            <ImplicitParameter as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Constraint as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Constraint(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Constraints<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constraint(inner) => inner.span(),
            Self::Context(inner) => inner.span(),
            Self::Forall(inner) => inner.span(),
            Self::ImplicitParameter(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decl<'tree> {
    Bind(::std::boxed::Box<Bind<'tree>>),
    Function(::std::boxed::Box<Function<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Decl<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bind" => {
                Ok(
                    Self::Bind(
                        ::std::boxed::Box::new(
                            <Bind as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "function" => {
                Ok(
                    Self::Function(
                        ::std::boxed::Box::new(
                            <Function as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Decl<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bind(inner) => inner.span(),
            Self::Function(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<'tree> {
    Class(::std::boxed::Box<Class<'tree>>),
    DataFamily(::std::boxed::Box<DataFamily<'tree>>),
    DataInstance(::std::boxed::Box<DataInstance<'tree>>),
    DataType(::std::boxed::Box<DataType<'tree>>),
    Decl(::std::boxed::Box<Decl<'tree>>),
    DefaultTypes(::std::boxed::Box<DefaultTypes<'tree>>),
    DerivingInstance(::std::boxed::Box<DerivingInstance<'tree>>),
    Fixity(::std::boxed::Box<Fixity<'tree>>),
    ForeignExport(::std::boxed::Box<ForeignExport<'tree>>),
    ForeignImport(::std::boxed::Box<ForeignImport<'tree>>),
    Instance(::std::boxed::Box<Instance<'tree>>),
    KindSignature(::std::boxed::Box<KindSignature<'tree>>),
    Newtype(::std::boxed::Box<Newtype<'tree>>),
    PatternSynonym(::std::boxed::Box<PatternSynonym<'tree>>),
    RoleAnnotation(::std::boxed::Box<RoleAnnotation<'tree>>),
    TopSplice(::std::boxed::Box<TopSplice<'tree>>),
    TypeFamily(::std::boxed::Box<TypeFamily<'tree>>),
    TypeInstance(::std::boxed::Box<TypeInstance<'tree>>),
    TypeSynomym(::std::boxed::Box<TypeSynomym<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declaration<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class" => {
                Ok(
                    Self::Class(
                        ::std::boxed::Box::new(
                            <Class as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "data_family" => {
                Ok(
                    Self::DataFamily(
                        ::std::boxed::Box::new(
                            <DataFamily as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "data_instance" => {
                Ok(
                    Self::DataInstance(
                        ::std::boxed::Box::new(
                            <DataInstance as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "data_type" => {
                Ok(
                    Self::DataType(
                        ::std::boxed::Box::new(
                            <DataType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "default_types" => {
                Ok(
                    Self::DefaultTypes(
                        ::std::boxed::Box::new(
                            <DefaultTypes as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "deriving_instance" => {
                Ok(
                    Self::DerivingInstance(
                        ::std::boxed::Box::new(
                            <DerivingInstance as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "fixity" => {
                Ok(
                    Self::Fixity(
                        ::std::boxed::Box::new(
                            <Fixity as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "foreign_export" => {
                Ok(
                    Self::ForeignExport(
                        ::std::boxed::Box::new(
                            <ForeignExport as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "foreign_import" => {
                Ok(
                    Self::ForeignImport(
                        ::std::boxed::Box::new(
                            <ForeignImport as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "instance" => {
                Ok(
                    Self::Instance(
                        ::std::boxed::Box::new(
                            <Instance as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "kind_signature" => {
                Ok(
                    Self::KindSignature(
                        ::std::boxed::Box::new(
                            <KindSignature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "newtype" => {
                Ok(
                    Self::Newtype(
                        ::std::boxed::Box::new(
                            <Newtype as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pattern_synonym" => {
                Ok(
                    Self::PatternSynonym(
                        ::std::boxed::Box::new(
                            <PatternSynonym as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "role_annotation" => {
                Ok(
                    Self::RoleAnnotation(
                        ::std::boxed::Box::new(
                            <RoleAnnotation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "top_splice" => {
                Ok(
                    Self::TopSplice(
                        ::std::boxed::Box::new(
                            <TopSplice as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_family" => {
                Ok(
                    Self::TypeFamily(
                        ::std::boxed::Box::new(
                            <TypeFamily as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_instance" => {
                Ok(
                    Self::TypeInstance(
                        ::std::boxed::Box::new(
                            <TypeInstance as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_synomym" => {
                Ok(
                    Self::TypeSynomym(
                        ::std::boxed::Box::new(
                            <TypeSynomym as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Decl as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Decl(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Declaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Class(inner) => inner.span(),
            Self::DataFamily(inner) => inner.span(),
            Self::DataInstance(inner) => inner.span(),
            Self::DataType(inner) => inner.span(),
            Self::Decl(inner) => inner.span(),
            Self::DefaultTypes(inner) => inner.span(),
            Self::DerivingInstance(inner) => inner.span(),
            Self::Fixity(inner) => inner.span(),
            Self::ForeignExport(inner) => inner.span(),
            Self::ForeignImport(inner) => inner.span(),
            Self::Instance(inner) => inner.span(),
            Self::KindSignature(inner) => inner.span(),
            Self::Newtype(inner) => inner.span(),
            Self::PatternSynonym(inner) => inner.span(),
            Self::RoleAnnotation(inner) => inner.span(),
            Self::TopSplice(inner) => inner.span(),
            Self::TypeFamily(inner) => inner.span(),
            Self::TypeInstance(inner) => inner.span(),
            Self::TypeSynomym(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'tree> {
    Apply(::std::boxed::Box<Apply<'tree>>),
    ArithmeticSequence(::std::boxed::Box<ArithmeticSequence<'tree>>),
    Case(::std::boxed::Box<Case<'tree>>),
    Conditional(::std::boxed::Box<Conditional<'tree>>),
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    Do(::std::boxed::Box<Do<'tree>>),
    ImplicitVariable(::std::boxed::Box<ImplicitVariable<'tree>>),
    Infix(::std::boxed::Box<Infix<'tree>>),
    Label(::std::boxed::Box<Label<'tree>>),
    Lambda(::std::boxed::Box<Lambda<'tree>>),
    LambdaCase(::std::boxed::Box<LambdaCase<'tree>>),
    LambdaCases(::std::boxed::Box<LambdaCases<'tree>>),
    LeftSection(::std::boxed::Box<LeftSection<'tree>>),
    LetIn(::std::boxed::Box<LetIn<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    ListComprehension(::std::boxed::Box<ListComprehension<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    MultiWayIf(::std::boxed::Box<MultiWayIf<'tree>>),
    Negation(::std::boxed::Box<Negation<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixTuple(::std::boxed::Box<PrefixTuple<'tree>>),
    PrefixUnboxedSum(::std::boxed::Box<PrefixUnboxedSum<'tree>>),
    PrefixUnboxedTuple(::std::boxed::Box<PrefixUnboxedTuple<'tree>>),
    Projection(::std::boxed::Box<Projection<'tree>>),
    ProjectionSelector(::std::boxed::Box<ProjectionSelector<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Quasiquote(::std::boxed::Box<Quasiquote<'tree>>),
    Quote(::std::boxed::Box<Quote<'tree>>),
    Record(::std::boxed::Box<Record<'tree>>),
    RightSection(::std::boxed::Box<RightSection<'tree>>),
    Splice(::std::boxed::Box<Splice<'tree>>),
    ThQuotedName(::std::boxed::Box<ThQuotedName<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    TypedQuote(::std::boxed::Box<TypedQuote<'tree>>),
    UnboxedSum(::std::boxed::Box<UnboxedSum<'tree>>),
    UnboxedTuple(::std::boxed::Box<UnboxedTuple<'tree>>),
    UnboxedUnit(::std::boxed::Box<UnboxedUnit<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "apply" => {
                Ok(
                    Self::Apply(
                        ::std::boxed::Box::new(
                            <Apply as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "arithmetic_sequence" => {
                Ok(
                    Self::ArithmeticSequence(
                        ::std::boxed::Box::new(
                            <ArithmeticSequence as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "case" => {
                Ok(
                    Self::Case(
                        ::std::boxed::Box::new(
                            <Case as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "conditional" => {
                Ok(
                    Self::Conditional(
                        ::std::boxed::Box::new(
                            <Conditional as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "do" => {
                Ok(
                    Self::Do(
                        ::std::boxed::Box::new(
                            <Do as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "implicit_variable" => {
                Ok(
                    Self::ImplicitVariable(
                        ::std::boxed::Box::new(
                            <ImplicitVariable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "label" => {
                Ok(
                    Self::Label(
                        ::std::boxed::Box::new(
                            <Label as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "lambda" => {
                Ok(
                    Self::Lambda(
                        ::std::boxed::Box::new(
                            <Lambda as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "lambda_case" => {
                Ok(
                    Self::LambdaCase(
                        ::std::boxed::Box::new(
                            <LambdaCase as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "lambda_cases" => {
                Ok(
                    Self::LambdaCases(
                        ::std::boxed::Box::new(
                            <LambdaCases as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "left_section" => {
                Ok(
                    Self::LeftSection(
                        ::std::boxed::Box::new(
                            <LeftSection as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "let_in" => {
                Ok(
                    Self::LetIn(
                        ::std::boxed::Box::new(
                            <LetIn as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list" => {
                Ok(
                    Self::List(
                        ::std::boxed::Box::new(
                            <List as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "list_comprehension" => {
                Ok(
                    Self::ListComprehension(
                        ::std::boxed::Box::new(
                            <ListComprehension as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "literal" => {
                Ok(
                    Self::Literal(
                        ::std::boxed::Box::new(
                            <Literal as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "multi_way_if" => {
                Ok(
                    Self::MultiWayIf(
                        ::std::boxed::Box::new(
                            <MultiWayIf as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "negation" => {
                Ok(
                    Self::Negation(
                        ::std::boxed::Box::new(
                            <Negation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_tuple" => {
                Ok(
                    Self::PrefixTuple(
                        ::std::boxed::Box::new(
                            <PrefixTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_sum" => {
                Ok(
                    Self::PrefixUnboxedSum(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_tuple" => {
                Ok(
                    Self::PrefixUnboxedTuple(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "projection" => {
                Ok(
                    Self::Projection(
                        ::std::boxed::Box::new(
                            <Projection as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "projection_selector" => {
                Ok(
                    Self::ProjectionSelector(
                        ::std::boxed::Box::new(
                            <ProjectionSelector as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quasiquote" => {
                Ok(
                    Self::Quasiquote(
                        ::std::boxed::Box::new(
                            <Quasiquote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quote" => {
                Ok(
                    Self::Quote(
                        ::std::boxed::Box::new(
                            <Quote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "record" => {
                Ok(
                    Self::Record(
                        ::std::boxed::Box::new(
                            <Record as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "right_section" => {
                Ok(
                    Self::RightSection(
                        ::std::boxed::Box::new(
                            <RightSection as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "splice" => {
                Ok(
                    Self::Splice(
                        ::std::boxed::Box::new(
                            <Splice as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "th_quoted_name" => {
                Ok(
                    Self::ThQuotedName(
                        ::std::boxed::Box::new(
                            <ThQuotedName as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tuple" => {
                Ok(
                    Self::Tuple(
                        ::std::boxed::Box::new(
                            <Tuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "typed_quote" => {
                Ok(
                    Self::TypedQuote(
                        ::std::boxed::Box::new(
                            <TypedQuote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_sum" => {
                Ok(
                    Self::UnboxedSum(
                        ::std::boxed::Box::new(
                            <UnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_tuple" => {
                Ok(
                    Self::UnboxedTuple(
                        ::std::boxed::Box::new(
                            <UnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_unit" => {
                Ok(
                    Self::UnboxedUnit(
                        ::std::boxed::Box::new(
                            <UnboxedUnit as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
            Self::Apply(inner) => inner.span(),
            Self::ArithmeticSequence(inner) => inner.span(),
            Self::Case(inner) => inner.span(),
            Self::Conditional(inner) => inner.span(),
            Self::Constructor(inner) => inner.span(),
            Self::Do(inner) => inner.span(),
            Self::ImplicitVariable(inner) => inner.span(),
            Self::Infix(inner) => inner.span(),
            Self::Label(inner) => inner.span(),
            Self::Lambda(inner) => inner.span(),
            Self::LambdaCase(inner) => inner.span(),
            Self::LambdaCases(inner) => inner.span(),
            Self::LeftSection(inner) => inner.span(),
            Self::LetIn(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::ListComprehension(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::MultiWayIf(inner) => inner.span(),
            Self::Negation(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixTuple(inner) => inner.span(),
            Self::PrefixUnboxedSum(inner) => inner.span(),
            Self::PrefixUnboxedTuple(inner) => inner.span(),
            Self::Projection(inner) => inner.span(),
            Self::ProjectionSelector(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Quasiquote(inner) => inner.span(),
            Self::Quote(inner) => inner.span(),
            Self::Record(inner) => inner.span(),
            Self::RightSection(inner) => inner.span(),
            Self::Splice(inner) => inner.span(),
            Self::ThQuotedName(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::TypedQuote(inner) => inner.span(),
            Self::UnboxedSum(inner) => inner.span(),
            Self::UnboxedTuple(inner) => inner.span(),
            Self::UnboxedUnit(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Guard<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Let(::std::boxed::Box<Let<'tree>>),
    PatternGuard(::std::boxed::Box<PatternGuard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Guard<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => {
                Ok(
                    Self::Boolean(
                        ::std::boxed::Box::new(
                            <Boolean as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "let" => {
                Ok(
                    Self::Let(
                        ::std::boxed::Box::new(
                            <Let as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "pattern_guard" => {
                Ok(
                    Self::PatternGuard(
                        ::std::boxed::Box::new(
                            <PatternGuard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Guard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::Let(inner) => inner.span(),
            Self::PatternGuard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceDecl<'tree> {
    DataInstance(::std::boxed::Box<DataInstance<'tree>>),
    Decl(::std::boxed::Box<Decl<'tree>>),
    TypeInstance(::std::boxed::Box<TypeInstance<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceDecl<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "data_instance" => {
                Ok(
                    Self::DataInstance(
                        ::std::boxed::Box::new(
                            <DataInstance as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_instance" => {
                Ok(
                    Self::TypeInstance(
                        ::std::boxed::Box::new(
                            <TypeInstance as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Decl as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Decl(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for InstanceDecl<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DataInstance(inner) => inner.span(),
            Self::Decl(inner) => inner.span(),
            Self::TypeInstance(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<'tree> {
    Apply(::std::boxed::Box<Apply<'tree>>),
    As(::std::boxed::Box<As<'tree>>),
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    Infix(::std::boxed::Box<Infix<'tree>>),
    Irrefutable(::std::boxed::Box<Irrefutable<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    Negation(::std::boxed::Box<Negation<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixTuple(::std::boxed::Box<PrefixTuple<'tree>>),
    PrefixUnboxedSum(::std::boxed::Box<PrefixUnboxedSum<'tree>>),
    PrefixUnboxedTuple(::std::boxed::Box<PrefixUnboxedTuple<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Quasiquote(::std::boxed::Box<Quasiquote<'tree>>),
    Record(::std::boxed::Box<Record<'tree>>),
    Splice(::std::boxed::Box<Splice<'tree>>),
    Strict(::std::boxed::Box<Strict<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnboxedSum(::std::boxed::Box<UnboxedSum<'tree>>),
    UnboxedTuple(::std::boxed::Box<UnboxedTuple<'tree>>),
    UnboxedUnit(::std::boxed::Box<UnboxedUnit<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "apply" => {
                Ok(
                    Self::Apply(
                        ::std::boxed::Box::new(
                            <Apply as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "as" => {
                Ok(
                    Self::As(
                        ::std::boxed::Box::new(
                            <As as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "irrefutable" => {
                Ok(
                    Self::Irrefutable(
                        ::std::boxed::Box::new(
                            <Irrefutable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list" => {
                Ok(
                    Self::List(
                        ::std::boxed::Box::new(
                            <List as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "literal" => {
                Ok(
                    Self::Literal(
                        ::std::boxed::Box::new(
                            <Literal as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "negation" => {
                Ok(
                    Self::Negation(
                        ::std::boxed::Box::new(
                            <Negation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_tuple" => {
                Ok(
                    Self::PrefixTuple(
                        ::std::boxed::Box::new(
                            <PrefixTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_sum" => {
                Ok(
                    Self::PrefixUnboxedSum(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_tuple" => {
                Ok(
                    Self::PrefixUnboxedTuple(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quasiquote" => {
                Ok(
                    Self::Quasiquote(
                        ::std::boxed::Box::new(
                            <Quasiquote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "record" => {
                Ok(
                    Self::Record(
                        ::std::boxed::Box::new(
                            <Record as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "splice" => {
                Ok(
                    Self::Splice(
                        ::std::boxed::Box::new(
                            <Splice as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict" => {
                Ok(
                    Self::Strict(
                        ::std::boxed::Box::new(
                            <Strict as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tuple" => {
                Ok(
                    Self::Tuple(
                        ::std::boxed::Box::new(
                            <Tuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_sum" => {
                Ok(
                    Self::UnboxedSum(
                        ::std::boxed::Box::new(
                            <UnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_tuple" => {
                Ok(
                    Self::UnboxedTuple(
                        ::std::boxed::Box::new(
                            <UnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_unit" => {
                Ok(
                    Self::UnboxedUnit(
                        ::std::boxed::Box::new(
                            <UnboxedUnit as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "wildcard" => {
                Ok(
                    Self::Wildcard(
                        ::std::boxed::Box::new(
                            <Wildcard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Apply(inner) => inner.span(),
            Self::As(inner) => inner.span(),
            Self::Constructor(inner) => inner.span(),
            Self::Infix(inner) => inner.span(),
            Self::Irrefutable(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::Negation(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixTuple(inner) => inner.span(),
            Self::PrefixUnboxedSum(inner) => inner.span(),
            Self::PrefixUnboxedTuple(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Quasiquote(inner) => inner.span(),
            Self::Record(inner) => inner.span(),
            Self::Splice(inner) => inner.span(),
            Self::Strict(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnboxedSum(inner) => inner.span(),
            Self::UnboxedTuple(inner) => inner.span(),
            Self::UnboxedUnit(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Qualifier<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Generator(::std::boxed::Box<Generator<'tree>>),
    Group(::std::boxed::Box<Group<'tree>>),
    Let(::std::boxed::Box<Let<'tree>>),
    Transform(::std::boxed::Box<Transform<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Qualifier<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => {
                Ok(
                    Self::Boolean(
                        ::std::boxed::Box::new(
                            <Boolean as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "generator" => {
                Ok(
                    Self::Generator(
                        ::std::boxed::Box::new(
                            <Generator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "group" => {
                Ok(
                    Self::Group(
                        ::std::boxed::Box::new(
                            <Group as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "let" => {
                Ok(
                    Self::Let(
                        ::std::boxed::Box::new(
                            <Let as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "transform" => {
                Ok(
                    Self::Transform(
                        ::std::boxed::Box::new(
                            <Transform as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Qualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::Generator(inner) => inner.span(),
            Self::Group(inner) => inner.span(),
            Self::Let(inner) => inner.span(),
            Self::Transform(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantifiedType<'tree> {
    Context(::std::boxed::Box<Context<'tree>>),
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
    Function(::std::boxed::Box<Function<'tree>>),
    ImplicitParameter(::std::boxed::Box<ImplicitParameter<'tree>>),
    LinearFunction(::std::boxed::Box<LinearFunction<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuantifiedType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "context" => {
                Ok(
                    Self::Context(
                        ::std::boxed::Box::new(
                            <Context as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "function" => {
                Ok(
                    Self::Function(
                        ::std::boxed::Box::new(
                            <Function as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "implicit_parameter" => {
                Ok(
                    Self::ImplicitParameter(
                        ::std::boxed::Box::new(
                            <ImplicitParameter as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "linear_function" => {
                Ok(
                    Self::LinearFunction(
                        ::std::boxed::Box::new(
                            <LinearFunction as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QuantifiedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Context(inner) => inner.span(),
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
            Self::Function(inner) => inner.span(),
            Self::ImplicitParameter(inner) => inner.span(),
            Self::LinearFunction(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'tree> {
    Bind(::std::boxed::Box<Bind<'tree>>),
    Exp(::std::boxed::Box<Exp<'tree>>),
    Let(::std::boxed::Box<Let<'tree>>),
    Rec(::std::boxed::Box<Rec<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bind" => {
                Ok(
                    Self::Bind(
                        ::std::boxed::Box::new(
                            <Bind as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "exp" => {
                Ok(
                    Self::Exp(
                        ::std::boxed::Box::new(
                            <Exp as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "let" => {
                Ok(
                    Self::Let(
                        ::std::boxed::Box::new(
                            <Let as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "rec" => {
                Ok(
                    Self::Rec(
                        ::std::boxed::Box::new(
                            <Rec as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Statement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bind(inner) => inner.span(),
            Self::Exp(inner) => inner.span(),
            Self::Let(inner) => inner.span(),
            Self::Rec(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'tree> {
    Apply(::std::boxed::Box<Apply<'tree>>),
    Infix(::std::boxed::Box<Infix<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    PrefixTuple(::std::boxed::Box<PrefixTuple<'tree>>),
    PrefixUnboxedSum(::std::boxed::Box<PrefixUnboxedSum<'tree>>),
    PrefixUnboxedTuple(::std::boxed::Box<PrefixUnboxedTuple<'tree>>),
    Promoted(::std::boxed::Box<Promoted<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Quasiquote(::std::boxed::Box<Quasiquote<'tree>>),
    Splice(::std::boxed::Box<Splice<'tree>>),
    Star(::std::boxed::Box<Star<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnboxedSum(::std::boxed::Box<UnboxedSum<'tree>>),
    UnboxedTuple(::std::boxed::Box<UnboxedTuple<'tree>>),
    UnboxedUnit(::std::boxed::Box<UnboxedUnit<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "apply" => {
                Ok(
                    Self::Apply(
                        ::std::boxed::Box::new(
                            <Apply as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list" => {
                Ok(
                    Self::List(
                        ::std::boxed::Box::new(
                            <List as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "literal" => {
                Ok(
                    Self::Literal(
                        ::std::boxed::Box::new(
                            <Literal as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_tuple" => {
                Ok(
                    Self::PrefixTuple(
                        ::std::boxed::Box::new(
                            <PrefixTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_sum" => {
                Ok(
                    Self::PrefixUnboxedSum(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_unboxed_tuple" => {
                Ok(
                    Self::PrefixUnboxedTuple(
                        ::std::boxed::Box::new(
                            <PrefixUnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "promoted" => {
                Ok(
                    Self::Promoted(
                        ::std::boxed::Box::new(
                            <Promoted as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quasiquote" => {
                Ok(
                    Self::Quasiquote(
                        ::std::boxed::Box::new(
                            <Quasiquote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "splice" => {
                Ok(
                    Self::Splice(
                        ::std::boxed::Box::new(
                            <Splice as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "star" => {
                Ok(
                    Self::Star(
                        ::std::boxed::Box::new(
                            <Star as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "tuple" => {
                Ok(
                    Self::Tuple(
                        ::std::boxed::Box::new(
                            <Tuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_sum" => {
                Ok(
                    Self::UnboxedSum(
                        ::std::boxed::Box::new(
                            <UnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_tuple" => {
                Ok(
                    Self::UnboxedTuple(
                        ::std::boxed::Box::new(
                            <UnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_unit" => {
                Ok(
                    Self::UnboxedUnit(
                        ::std::boxed::Box::new(
                            <UnboxedUnit as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "wildcard" => {
                Ok(
                    Self::Wildcard(
                        ::std::boxed::Box::new(
                            <Wildcard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Type<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Apply(inner) => inner.span(),
            Self::Infix(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::PrefixTuple(inner) => inner.span(),
            Self::PrefixUnboxedSum(inner) => inner.span(),
            Self::PrefixUnboxedTuple(inner) => inner.span(),
            Self::Promoted(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Quasiquote(inner) => inner.span(),
            Self::Splice(inner) => inner.span(),
            Self::Star(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnboxedSum(inner) => inner.span(),
            Self::UnboxedTuple(inner) => inner.span(),
            Self::UnboxedUnit(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeParam<'tree> {
    Invisible(::std::boxed::Box<Invisible<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParam<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "invisible" => {
                Ok(
                    Self::Invisible(
                        ::std::boxed::Box::new(
                            <Invisible as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "wildcard" => {
                Ok(
                    Self::Wildcard(
                        ::std::boxed::Box::new(
                            <Wildcard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeParam<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Invisible(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractFamily<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractFamily<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_family");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AbstractFamily<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AbstractFamily<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alternative<'tree> {
    pub span: ::treesitter_types::Span,
    pub binds: ::core::option::Option<LocalBinds<'tree>>,
    pub r#match: ::std::vec::Vec<Match<'tree>>,
    pub pattern: ::core::option::Option<AlternativePattern<'tree>>,
    pub patterns: ::core::option::Option<Patterns<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Alternative<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alternative");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            binds: match node.child_by_field_name("binds") {
                Some(child) => {
                    Some(
                        <LocalBinds as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#match: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("match", &mut cursor) {
                    items
                        .push(
                            <Match as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => {
                    Some(
                        <AlternativePattern as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <Patterns as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Alternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alternatives<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::std::vec::Vec<Alternative<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Alternatives<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alternatives");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("alternative", &mut cursor) {
                    items
                        .push(
                            <Alternative as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Alternatives<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotated<'tree> {
    pub span: ::treesitter_types::Span,
    pub kind: QuantifiedType<'tree>,
    pub children: TypeParam<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Annotated<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotated");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            kind: {
                let child = node
                    .child_by_field_name("kind")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "kind",
                        node,
                    ))?;
                <QuantifiedType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <TypeParam as ::treesitter_types::FromNode>::from_node(
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
                                            <TypeParam as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <TypeParam as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Annotated<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Apply<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ApplyArgument<'tree>,
    pub constructor: ::core::option::Option<ApplyConstructor<'tree>>,
    pub function: ::core::option::Option<ApplyFunction<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Apply<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "apply");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node
                    .child_by_field_name("argument")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "argument",
                        node,
                    ))?;
                <ApplyArgument as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            constructor: match node.child_by_field_name("constructor") {
                Some(child) => {
                    Some(
                        <ApplyConstructor as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            function: match node.child_by_field_name("function") {
                Some(child) => {
                    Some(
                        <ApplyFunction as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Apply<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArithmeticSequence<'tree> {
    pub span: ::treesitter_types::Span,
    pub from: ArithmeticSequenceFrom<'tree>,
    pub step: ::core::option::Option<ArithmeticSequenceStep<'tree>>,
    pub to: ::core::option::Option<ArithmeticSequenceTo<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArithmeticSequence<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "arithmetic_sequence");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            from: {
                let child = node
                    .child_by_field_name("from")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "from",
                        node,
                    ))?;
                <ArithmeticSequenceFrom as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            step: match node.child_by_field_name("step") {
                Some(child) => {
                    Some(
                        <ArithmeticSequenceStep as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            to: match node.child_by_field_name("to") {
                Some(child) => {
                    Some(
                        <ArithmeticSequenceTo as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArithmeticSequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct As<'tree> {
    pub span: ::treesitter_types::Span,
    pub bind: Variable<'tree>,
    pub pattern: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for As<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "as");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bind: {
                let child = node
                    .child_by_field_name("bind")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "bind",
                        node,
                    ))?;
                <Variable as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            pattern: {
                let child = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "pattern",
                        node,
                    ))?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for As<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociatedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub namespace: AssociatedTypeNamespace,
    pub r#type: AssociatedTypeType<'tree>,
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
            namespace: {
                let child = node
                    .child_by_field_name("namespace")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "namespace",
                        node,
                    ))?;
                <AssociatedTypeNamespace as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <AssociatedTypeType as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
pub struct Bind<'tree> {
    pub span: ::treesitter_types::Span,
    pub arrow: ::core::option::Option<BindArrow>,
    pub binds: ::core::option::Option<LocalBinds<'tree>>,
    pub expression: ::core::option::Option<BindExpression<'tree>>,
    pub implicit: ::core::option::Option<ImplicitVariable<'tree>>,
    pub r#match: ::std::vec::Vec<Match<'tree>>,
    pub name: ::core::option::Option<BindName<'tree>>,
    pub pattern: ::core::option::Option<BindPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Bind<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bind");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arrow: match node.child_by_field_name("arrow") {
                Some(child) => {
                    Some(
                        <BindArrow as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            binds: match node.child_by_field_name("binds") {
                Some(child) => {
                    Some(
                        <LocalBinds as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            expression: match node.child_by_field_name("expression") {
                Some(child) => {
                    Some(
                        <BindExpression as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            implicit: match node.child_by_field_name("implicit") {
                Some(child) => {
                    Some(
                        <ImplicitVariable as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#match: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("match", &mut cursor) {
                    items
                        .push(
                            <Match as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <BindName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => {
                    Some(
                        <BindPattern as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Bind<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingList<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<BindingListName<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindingList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binding_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <BindingListName as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BindingList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: BooleanChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Boolean<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boolean");
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
                                        <BooleanChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <BooleanChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <BooleanChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Boolean<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternatives: ::core::option::Option<Alternatives<'tree>>,
    pub children: CaseChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Case<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternatives: match node.child_by_field_name("alternatives") {
                Some(child) => {
                    Some(
                        <Alternatives as ::treesitter_types::FromNode>::from_node(
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
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <CaseChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <CaseChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <CaseChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Case<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Children<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: ::std::vec::Vec<ChildrenElement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Children<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "children");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            element: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("element", &mut cursor) {
                    items
                        .push(
                            <ChildrenElement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Children<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class<'tree> {
    pub span: ::treesitter_types::Span,
    pub context: ::core::option::Option<Context<'tree>>,
    pub declarations: ::core::option::Option<ClassDeclarations<'tree>>,
    pub fundeps: ::core::option::Option<Fundeps<'tree>>,
    pub name: ::core::option::Option<ClassName<'tree>>,
    pub patterns: ::core::option::Option<TypeParams<'tree>>,
    pub children: ::core::option::Option<ClassChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Class<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            context: match node.child_by_field_name("context") {
                Some(child) => {
                    Some(
                        <Context as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            declarations: match node.child_by_field_name("declarations") {
                Some(child) => {
                    Some(
                        <ClassDeclarations as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            fundeps: match node.child_by_field_name("fundeps") {
                Some(child) => {
                    Some(
                        <Fundeps as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <ClassName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypeParams as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <ClassChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Class<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassDeclarations<'tree> {
    pub span: ::treesitter_types::Span,
    pub declaration: ::std::vec::Vec<ClassDecl<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDeclarations<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_declarations");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declaration: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declaration", &mut cursor) {
                    items
                        .push(
                            <ClassDecl as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ClassDeclarations<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#else: ConditionalElse<'tree>,
    pub r#if: ConditionalIf<'tree>,
    pub then: ConditionalThen<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Conditional<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "conditional");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#else: {
                let child = node
                    .child_by_field_name("else")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "else",
                        node,
                    ))?;
                <ConditionalElse as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#if: {
                let child = node
                    .child_by_field_name("if")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "if",
                        node,
                    ))?;
                <ConditionalIf as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            then: {
                let child = node
                    .child_by_field_name("then")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "then",
                        node,
                    ))?;
                <ConditionalThen as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Conditional<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ConstructorOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ConstructorOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorSynonym<'tree> {
    pub span: ::treesitter_types::Span,
    pub binds: ::core::option::Option<LocalBinds<'tree>>,
    pub implicit: ::core::option::Option<ImplicitVariable<'tree>>,
    pub r#match: ::std::vec::Vec<Match<'tree>>,
    pub name: ::core::option::Option<ConstructorSynonymName<'tree>>,
    pub pattern: ::core::option::Option<ConstructorSynonymPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorSynonym<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_synonym");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            binds: match node.child_by_field_name("binds") {
                Some(child) => {
                    Some(
                        <LocalBinds as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            implicit: match node.child_by_field_name("implicit") {
                Some(child) => {
                    Some(
                        <ImplicitVariable as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#match: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("match", &mut cursor) {
                    items
                        .push(
                            <Match as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <ConstructorSynonymName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => {
                    Some(
                        <ConstructorSynonymPattern as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConstructorSynonym<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorSynonyms<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConstructorSynonym<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorSynonyms<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_synonyms");
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
                            <ConstructorSynonym as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConstructorSynonyms<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context<'tree> {
    pub span: ::treesitter_types::Span,
    pub arrow: ContextArrow,
    pub constraint: ::core::option::Option<Constraints<'tree>>,
    pub context: Constraint<'tree>,
    pub r#type: ::core::option::Option<QuantifiedType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Context<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "context");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arrow: {
                let child = node
                    .child_by_field_name("arrow")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "arrow",
                        node,
                    ))?;
                <ContextArrow as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            constraint: match node.child_by_field_name("constraint") {
                Some(child) => {
                    Some(
                        <Constraints as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            context: {
                let child = node
                    .child_by_field_name("context")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "context",
                        node,
                    ))?;
                <Constraint as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Context<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataConstructor<'tree> {
    pub span: ::treesitter_types::Span,
    pub constructor: DataConstructorConstructor<'tree>,
    pub context: ::core::option::Option<Context<'tree>>,
    pub forall: ::core::option::Option<DataConstructorForall<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataConstructor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "data_constructor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constructor: {
                let child = node
                    .child_by_field_name("constructor")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "constructor",
                        node,
                    ))?;
                <DataConstructorConstructor as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            context: match node.child_by_field_name("context") {
                Some(child) => {
                    Some(
                        <Context as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <DataConstructorForall as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataConstructors<'tree> {
    pub span: ::treesitter_types::Span,
    pub constructor: ::std::vec::Vec<DataConstructor<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataConstructors<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "data_constructors");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constructor: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("constructor", &mut cursor) {
                    items
                        .push(
                            <DataConstructor as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataConstructors<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFamily<'tree> {
    pub span: ::treesitter_types::Span,
    pub kind: ::core::option::Option<QuantifiedType<'tree>>,
    pub name: ::core::option::Option<DataFamilyName<'tree>>,
    pub patterns: ::core::option::Option<TypeParams<'tree>>,
    pub children: ::core::option::Option<DataFamilyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataFamily<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "data_family");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            kind: match node.child_by_field_name("kind") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <DataFamilyName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypeParams as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <DataFamilyChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataFamily<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataInstance<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: DataInstanceChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataInstance<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "data_instance");
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
                                        <DataInstanceChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <DataInstanceChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <DataInstanceChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DataInstance<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataType<'tree> {
    pub span: ::treesitter_types::Span,
    pub constructors: ::core::option::Option<DataTypeConstructors<'tree>>,
    pub context: ::core::option::Option<Context<'tree>>,
    pub deriving: ::std::vec::Vec<Deriving<'tree>>,
    pub forall: ::core::option::Option<DataTypeForall<'tree>>,
    pub kind: ::core::option::Option<QuantifiedType<'tree>>,
    pub name: ::core::option::Option<DataTypeName<'tree>>,
    pub patterns: ::core::option::Option<DataTypePatterns<'tree>>,
    pub children: ::core::option::Option<DataTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "data_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constructors: match node.child_by_field_name("constructors") {
                Some(child) => {
                    Some(
                        <DataTypeConstructors as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            context: match node.child_by_field_name("context") {
                Some(child) => {
                    Some(
                        <Context as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            deriving: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("deriving", &mut cursor) {
                    items
                        .push(
                            <Deriving as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <DataTypeForall as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            kind: match node.child_by_field_name("kind") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <DataTypeName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <DataTypePatterns as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <DataTypeChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declarations<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeclarationsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declarations<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declarations");
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
                            <DeclarationsChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Declarations<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultSignature<'tree> {
    pub span: ::treesitter_types::Span,
    pub signature: Signature<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultSignature<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "default_signature");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            signature: {
                let child = node
                    .child_by_field_name("signature")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "signature",
                        node,
                    ))?;
                <Signature as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DefaultSignature<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultTypes<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::std::vec::Vec<DefaultTypesType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultTypes<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "default_types");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items
                        .push(
                            <DefaultTypesType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DefaultTypes<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deriving<'tree> {
    pub span: ::treesitter_types::Span,
    pub classes: Constraint<'tree>,
    pub strategy: ::core::option::Option<DerivingStrategy<'tree>>,
    pub via: ::core::option::Option<Via<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Deriving<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "deriving");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            classes: {
                let child = node
                    .child_by_field_name("classes")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "classes",
                        node,
                    ))?;
                <Constraint as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            strategy: match node.child_by_field_name("strategy") {
                Some(child) => {
                    Some(
                        <DerivingStrategy as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            via: match node.child_by_field_name("via") {
                Some(child) => {
                    Some(<Via as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Deriving<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivingInstance<'tree> {
    pub span: ::treesitter_types::Span,
    pub context: ::core::option::Option<Context<'tree>>,
    pub forall: ::core::option::Option<DerivingInstanceForall<'tree>>,
    pub name: ::core::option::Option<DerivingInstanceName<'tree>>,
    pub patterns: ::core::option::Option<TypePatterns<'tree>>,
    pub strategy: ::core::option::Option<DerivingStrategy<'tree>>,
    pub via: ::core::option::Option<Via<'tree>>,
    pub children: ::core::option::Option<DerivingInstanceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DerivingInstance<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "deriving_instance");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            context: match node.child_by_field_name("context") {
                Some(child) => {
                    Some(
                        <Context as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <DerivingInstanceForall as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <DerivingInstanceName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypePatterns as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            strategy: match node.child_by_field_name("strategy") {
                Some(child) => {
                    Some(
                        <DerivingStrategy as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            via: match node.child_by_field_name("via") {
                Some(child) => {
                    Some(<Via as ::treesitter_types::FromNode>::from_node(child, src)?)
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <DerivingInstanceChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DerivingInstance<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivingStrategy<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DerivingStrategy<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "deriving_strategy");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DerivingStrategy<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DerivingStrategy<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Do<'tree> {
    pub span: ::treesitter_types::Span,
    pub statement: ::std::vec::Vec<Statement<'tree>>,
    pub children: ::core::option::Option<DoModule<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Do<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            statement: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("statement", &mut cursor) {
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
                            <DoModule as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Do<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoModule<'tree> {
    pub span: ::treesitter_types::Span,
    pub id: DoModuleId,
    pub module: Module<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoModule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do_module");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            id: {
                let child = node
                    .child_by_field_name("id")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "id",
                        node,
                    ))?;
                <DoModuleId as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "module",
                        node,
                    ))?;
                <Module as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DoModule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyList<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EmptyList<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "empty_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for EmptyList<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for EmptyList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: String<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Entity<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "entity");
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
                                        <String as ::treesitter_types::FromNode>::from_node(
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
                                            <String as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <String as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Entity<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equation<'tree> {
    pub span: ::treesitter_types::Span,
    pub constructors: ::core::option::Option<ConstructorSynonyms<'tree>>,
    pub forall: ::core::option::Option<EquationForall<'tree>>,
    pub name: ::core::option::Option<EquationName<'tree>>,
    pub pattern: ::core::option::Option<EquationPattern<'tree>>,
    pub patterns: ::core::option::Option<TypePatterns<'tree>>,
    pub synonym: ::core::option::Option<Pattern<'tree>>,
    pub children: ::std::vec::Vec<EquationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Equation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "equation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constructors: match node.child_by_field_name("constructors") {
                Some(child) => {
                    Some(
                        <ConstructorSynonyms as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <EquationForall as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <EquationName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => {
                    Some(
                        <EquationPattern as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypePatterns as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            synonym: match node.child_by_field_name("synonym") {
                Some(child) => {
                    Some(
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                            <EquationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Equation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equations<'tree> {
    pub span: ::treesitter_types::Span,
    pub equation: ::std::vec::Vec<Equation<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Equations<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "equations");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            equation: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("equation", &mut cursor) {
                    items
                        .push(
                            <Equation as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Equations<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exp<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ExpChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Exp<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exp");
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
                                        <ExpChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <ExpChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <ExpChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Exp<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExplicitType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "explicit_type");
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
        })
    }
}
impl ::treesitter_types::Spanned for ExplicitType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Export<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Children<'tree>>,
    pub namespace: ::core::option::Option<Namespace<'tree>>,
    pub operator: ::core::option::Option<PrefixId<'tree>>,
    pub r#type: ::core::option::Option<ExportType<'tree>>,
    pub variable: ::core::option::Option<ExportVariable<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Export<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "export");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: match node.child_by_field_name("children") {
                Some(child) => {
                    Some(
                        <Children as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            namespace: match node.child_by_field_name("namespace") {
                Some(child) => {
                    Some(
                        <Namespace as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => {
                    Some(
                        <PrefixId as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <ExportType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            variable: match node.child_by_field_name("variable") {
                Some(child) => {
                    Some(
                        <ExportVariable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Export<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exports<'tree> {
    pub span: ::treesitter_types::Span,
    pub export: ::std::vec::Vec<Export<'tree>>,
    pub children: ::std::vec::Vec<ModuleExport<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Exports<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exports");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            export: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("export", &mut cursor) {
                    items
                        .push(
                            <Export as ::treesitter_types::FromNode>::from_node(
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items
                        .push(
                            <ModuleExport as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Exports<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<FieldName<'tree>>,
    pub parameter: ::core::option::Option<FieldParameter<'tree>>,
    pub r#type: ::core::option::Option<FieldType<'tree>>,
    pub children: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Field<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <FieldName as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            parameter: match node.child_by_field_name("parameter") {
                Some(child) => {
                    Some(
                        <FieldParameter as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <FieldType as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <Type as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Field<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Variable<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_name");
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
                                        <Variable as ::treesitter_types::FromNode>::from_node(
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
                                            <Variable as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Variable as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPath<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: FieldPathField<'tree>,
    pub subfield: ::std::vec::Vec<FieldName<'tree>>,
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
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "field",
                        node,
                    ))?;
                <FieldPathField as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            subfield: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("subfield", &mut cursor) {
                    items
                        .push(
                            <FieldName as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: ::core::option::Option<FieldPatternField<'tree>>,
    pub pattern: ::core::option::Option<FieldPatternPattern<'tree>>,
    pub children: ::core::option::Option<Wildcard<'tree>>,
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
            field: match node.child_by_field_name("field") {
                Some(child) => {
                    Some(
                        <FieldPatternField as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => {
                    Some(
                        <FieldPatternPattern as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <Wildcard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldUpdate<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ::core::option::Option<FieldUpdateExpression<'tree>>,
    pub field: ::core::option::Option<FieldUpdateField<'tree>>,
    pub children: ::core::option::Option<Wildcard<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldUpdate<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_update");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: match node.child_by_field_name("expression") {
                Some(child) => {
                    Some(
                        <FieldUpdateExpression as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            field: match node.child_by_field_name("field") {
                Some(child) => {
                    Some(
                        <FieldUpdateField as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <Wildcard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldUpdate<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fields<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: ::std::vec::Vec<Field<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Fields<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fields");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("field", &mut cursor) {
                    items
                        .push(
                            <Field as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Fields<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixity<'tree> {
    pub span: ::treesitter_types::Span,
    pub associativity: FixityAssociativity,
    pub operator: ::std::vec::Vec<FixityOperator<'tree>>,
    pub precedence: ::core::option::Option<Integer<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Fixity<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fixity");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            associativity: {
                let child = node
                    .child_by_field_name("associativity")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "associativity",
                        node,
                    ))?;
                <FixityAssociativity as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            operator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operator", &mut cursor) {
                    items
                        .push(
                            <FixityOperator as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            precedence: match node.child_by_field_name("precedence") {
                Some(child) => {
                    Some(
                        <Integer as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Fixity<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Forall<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: ::core::option::Option<Constraints<'tree>>,
    pub quantifier: ForallQuantifier,
    pub r#type: ::core::option::Option<QuantifiedType<'tree>>,
    pub variables: ::core::option::Option<QuantifiedVariables<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Forall<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "forall");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: match node.child_by_field_name("constraint") {
                Some(child) => {
                    Some(
                        <Constraints as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            quantifier: {
                let child = node
                    .child_by_field_name("quantifier")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "quantifier",
                        node,
                    ))?;
                <ForallQuantifier as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            variables: match node.child_by_field_name("variables") {
                Some(child) => {
                    Some(
                        <QuantifiedVariables as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Forall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForallRequired<'tree> {
    pub span: ::treesitter_types::Span,
    pub quantifier: ForallRequiredQuantifier,
    pub r#type: ::core::option::Option<QuantifiedType<'tree>>,
    pub variables: ::core::option::Option<QuantifiedVariables<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForallRequired<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "forall_required");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            quantifier: {
                let child = node
                    .child_by_field_name("quantifier")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "quantifier",
                        node,
                    ))?;
                <ForallRequiredQuantifier as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            variables: match node.child_by_field_name("variables") {
                Some(child) => {
                    Some(
                        <QuantifiedVariables as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ForallRequired<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignExport<'tree> {
    pub span: ::treesitter_types::Span,
    pub calling_convention: CallingConvention<'tree>,
    pub entity: ::core::option::Option<Entity<'tree>>,
    pub signature: Signature<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForeignExport<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "foreign_export");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            calling_convention: {
                let child = node
                    .child_by_field_name("calling_convention")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "calling_convention",
                        node,
                    ))?;
                <CallingConvention as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            entity: match node.child_by_field_name("entity") {
                Some(child) => {
                    Some(
                        <Entity as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            signature: {
                let child = node
                    .child_by_field_name("signature")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "signature",
                        node,
                    ))?;
                <Signature as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForeignExport<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignImport<'tree> {
    pub span: ::treesitter_types::Span,
    pub calling_convention: CallingConvention<'tree>,
    pub entity: ::core::option::Option<Entity<'tree>>,
    pub safety: ::core::option::Option<Safety<'tree>>,
    pub signature: Signature<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForeignImport<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "foreign_import");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            calling_convention: {
                let child = node
                    .child_by_field_name("calling_convention")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "calling_convention",
                        node,
                    ))?;
                <CallingConvention as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            entity: match node.child_by_field_name("entity") {
                Some(child) => {
                    Some(
                        <Entity as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            safety: match node.child_by_field_name("safety") {
                Some(child) => {
                    Some(
                        <Safety as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            signature: {
                let child = node
                    .child_by_field_name("signature")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "signature",
                        node,
                    ))?;
                <Signature as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForeignImport<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function<'tree> {
    pub span: ::treesitter_types::Span,
    pub arrow: ::core::option::Option<FunctionArrow>,
    pub binds: ::core::option::Option<LocalBinds<'tree>>,
    pub r#match: ::std::vec::Vec<Match<'tree>>,
    pub name: ::core::option::Option<FunctionName<'tree>>,
    pub parameter: ::core::option::Option<FunctionParameter<'tree>>,
    pub parens: ::core::option::Option<FunctionHeadParens<'tree>>,
    pub patterns: ::core::option::Option<Patterns<'tree>>,
    pub result: ::core::option::Option<QuantifiedType<'tree>>,
    pub children: ::core::option::Option<Infix<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Function<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arrow: match node.child_by_field_name("arrow") {
                Some(child) => {
                    Some(
                        <FunctionArrow as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            binds: match node.child_by_field_name("binds") {
                Some(child) => {
                    Some(
                        <LocalBinds as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#match: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("match", &mut cursor) {
                    items
                        .push(
                            <Match as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <FunctionName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            parameter: match node.child_by_field_name("parameter") {
                Some(child) => {
                    Some(
                        <FunctionParameter as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            parens: match node.child_by_field_name("parens") {
                Some(child) => {
                    Some(
                        <FunctionHeadParens as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <Patterns as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            result: match node.child_by_field_name("result") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <Infix as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Function<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionHeadParens<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<FunctionHeadParensName<'tree>>,
    pub parens: ::core::option::Option<::std::boxed::Box<FunctionHeadParens<'tree>>>,
    pub patterns: ::core::option::Option<Patterns<'tree>>,
    pub children: ::core::option::Option<Infix<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionHeadParens<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_head_parens");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <FunctionHeadParensName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            parens: match node.child_by_field_name("parens") {
                Some(child) => {
                    Some(
                        ::std::boxed::Box::new(
                            <FunctionHeadParens as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        ),
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <Patterns as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <Infix as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FunctionHeadParens<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fundep<'tree> {
    pub span: ::treesitter_types::Span,
    pub determined: ::std::vec::Vec<Variable<'tree>>,
    pub matched: ::std::vec::Vec<Variable<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Fundep<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fundep");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            determined: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("determined", &mut cursor) {
                    items
                        .push(
                            <Variable as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            matched: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("matched", &mut cursor) {
                    items
                        .push(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Fundep<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fundeps<'tree> {
    pub span: ::treesitter_types::Span,
    pub fundep: ::std::vec::Vec<Fundep<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Fundeps<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fundeps");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            fundep: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("fundep", &mut cursor) {
                    items
                        .push(
                            <Fundep as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Fundeps<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GadtConstructor<'tree> {
    pub span: ::treesitter_types::Span,
    pub context: ::core::option::Option<Context<'tree>>,
    pub forall: ::core::option::Option<GadtConstructorForall<'tree>>,
    pub name: ::core::option::Option<GadtConstructorName<'tree>>,
    pub names: ::core::option::Option<BindingList<'tree>>,
    pub r#type: GadtConstructorType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GadtConstructor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gadt_constructor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            context: match node.child_by_field_name("context") {
                Some(child) => {
                    Some(
                        <Context as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <GadtConstructorForall as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <GadtConstructorName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            names: match node.child_by_field_name("names") {
                Some(child) => {
                    Some(
                        <BindingList as ::treesitter_types::FromNode>::from_node(
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
                <GadtConstructorType as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GadtConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GadtConstructors<'tree> {
    pub span: ::treesitter_types::Span,
    pub constructor: ::std::vec::Vec<GadtConstructor<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GadtConstructors<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gadt_constructors");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constructor: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("constructor", &mut cursor) {
                    items
                        .push(
                            <GadtConstructor as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GadtConstructors<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generator<'tree> {
    pub span: ::treesitter_types::Span,
    pub arrow: GeneratorArrow,
    pub expression: GeneratorExpression<'tree>,
    pub pattern: GeneratorPattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Generator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arrow: {
                let child = node
                    .child_by_field_name("arrow")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "arrow",
                        node,
                    ))?;
                <GeneratorArrow as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <GeneratorExpression as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            pattern: {
                let child = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "pattern",
                        node,
                    ))?;
                <GeneratorPattern as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Generator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group<'tree> {
    pub span: ::treesitter_types::Span,
    pub classifier: GroupClassifier<'tree>,
    pub key: ::core::option::Option<GroupKey<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Group<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            classifier: {
                let child = node
                    .child_by_field_name("classifier")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "classifier",
                        node,
                    ))?;
                <GroupClassifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            key: match node.child_by_field_name("key") {
                Some(child) => {
                    Some(
                        <GroupKey as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Group<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guards<'tree> {
    pub span: ::treesitter_types::Span,
    pub guard: ::std::vec::Vec<Guard<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Guards<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "guards");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            guard: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("guard", &mut cursor) {
                    items
                        .push(
                            <Guard as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Guards<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Haskell<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarations: ::core::option::Option<Declarations<'tree>>,
    pub imports: ::core::option::Option<Imports<'tree>>,
    pub children: ::core::option::Option<Header<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Haskell<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "haskell");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarations: match node.child_by_field_name("declarations") {
                Some(child) => {
                    Some(
                        <Declarations as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            imports: match node.child_by_field_name("imports") {
                Some(child) => {
                    Some(
                        <Imports as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <Header as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Haskell<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'tree> {
    pub span: ::treesitter_types::Span,
    pub exports: ::core::option::Option<Exports<'tree>>,
    pub module: Module<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Header<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "header");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            exports: match node.child_by_field_name("exports") {
                Some(child) => {
                    Some(
                        <Exports as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "module",
                        node,
                    ))?;
                <Module as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Header<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplicitParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ImplicitVariable<'tree>,
    pub r#type: QuantifiedType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <ImplicitVariable as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <QuantifiedType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImplicitParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<Module<'tree>>,
    pub module: Module<'tree>,
    pub names: ::core::option::Option<ImportList<'tree>>,
    pub package: ::core::option::Option<ImportPackage<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Import<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: match node.child_by_field_name("alias") {
                Some(child) => {
                    Some(
                        <Module as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "module",
                        node,
                    ))?;
                <Module as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            names: match node.child_by_field_name("names") {
                Some(child) => {
                    Some(
                        <ImportList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            package: match node.child_by_field_name("package") {
                Some(child) => {
                    Some(
                        <ImportPackage as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Import<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportList<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<ImportName<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <ImportName as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Children<'tree>>,
    pub namespace: ::core::option::Option<Namespace<'tree>>,
    pub operator: ::core::option::Option<PrefixId<'tree>>,
    pub r#type: ::core::option::Option<ImportNameType<'tree>>,
    pub variable: ::core::option::Option<ImportNameVariable<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: match node.child_by_field_name("children") {
                Some(child) => {
                    Some(
                        <Children as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            namespace: match node.child_by_field_name("namespace") {
                Some(child) => {
                    Some(
                        <Namespace as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => {
                    Some(
                        <PrefixId as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <ImportNameType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            variable: match node.child_by_field_name("variable") {
                Some(child) => {
                    Some(
                        <ImportNameVariable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imports<'tree> {
    pub span: ::treesitter_types::Span,
    pub import: ::std::vec::Vec<Import<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Imports<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "imports");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            import: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("import", &mut cursor) {
                    items
                        .push(
                            <Import as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Imports<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inferred<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: InferredChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Inferred<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inferred");
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
                                        <InferredChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <InferredChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <InferredChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Inferred<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Infix<'tree> {
    pub span: ::treesitter_types::Span,
    pub left_operand: InfixLeftOperand<'tree>,
    pub operator: ::std::vec::Vec<InfixOperator<'tree>>,
    pub right_operand: InfixRightOperand<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Infix<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "infix");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left_operand: {
                let child = node
                    .child_by_field_name("left_operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "left_operand",
                        node,
                    ))?;
                <InfixLeftOperand as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            operator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operator", &mut cursor) {
                    items
                        .push(
                            <InfixOperator as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            right_operand: {
                let child = node
                    .child_by_field_name("right_operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "right_operand",
                        node,
                    ))?;
                <InfixRightOperand as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Infix<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfixId<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: InfixIdChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixId<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "infix_id");
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
                                        <InfixIdChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <InfixIdChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <InfixIdChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InfixId<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance<'tree> {
    pub span: ::treesitter_types::Span,
    pub context: ::core::option::Option<Context<'tree>>,
    pub declarations: ::core::option::Option<InstanceDeclarations<'tree>>,
    pub forall: ::core::option::Option<InstanceForall<'tree>>,
    pub name: ::core::option::Option<InstanceName<'tree>>,
    pub patterns: ::core::option::Option<TypePatterns<'tree>>,
    pub children: ::core::option::Option<InstanceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Instance<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            context: match node.child_by_field_name("context") {
                Some(child) => {
                    Some(
                        <Context as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            declarations: match node.child_by_field_name("declarations") {
                Some(child) => {
                    Some(
                        <InstanceDeclarations as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <InstanceForall as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <InstanceName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypePatterns as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <InstanceChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Instance<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceDeclarations<'tree> {
    pub span: ::treesitter_types::Span,
    pub declaration: ::std::vec::Vec<InstanceDecl<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceDeclarations<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance_declarations");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declaration: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declaration", &mut cursor) {
                    items
                        .push(
                            <InstanceDecl as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InstanceDeclarations<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invisible<'tree> {
    pub span: ::treesitter_types::Span,
    pub bind: TypeParam<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Invisible<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "invisible");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bind: {
                let child = node
                    .child_by_field_name("bind")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "bind",
                        node,
                    ))?;
                <TypeParam as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Invisible<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Irrefutable<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Irrefutable<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "irrefutable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "pattern",
                        node,
                    ))?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Irrefutable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindApplication<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KindApplication<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "kind_application");
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
        })
    }
}
impl ::treesitter_types::Spanned for KindApplication<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindSignature<'tree> {
    pub span: ::treesitter_types::Span,
    pub kind: QuantifiedType<'tree>,
    pub name: ::core::option::Option<KindSignatureName<'tree>>,
    pub patterns: ::core::option::Option<TypeParams<'tree>>,
    pub children: ::core::option::Option<KindSignatureChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KindSignature<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "kind_signature");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            kind: {
                let child = node
                    .child_by_field_name("kind")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "kind",
                        node,
                    ))?;
                <QuantifiedType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <KindSignatureName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypeParams as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <KindSignatureChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for KindSignature<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: LambdaExpression<'tree>,
    pub patterns: Patterns<'tree>,
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
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            patterns: {
                let child = node
                    .child_by_field_name("patterns")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "patterns",
                        node,
                    ))?;
                <Patterns as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Lambda<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternatives: ::core::option::Option<Alternatives<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternatives: match node.child_by_field_name("alternatives") {
                Some(child) => {
                    Some(
                        <Alternatives as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LambdaCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaCases<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternatives: ::core::option::Option<Alternatives<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaCases<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_cases");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternatives: match node.child_by_field_name("alternatives") {
                Some(child) => {
                    Some(
                        <Alternatives as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LambdaCases<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LazyField<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LazyField<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lazy_field");
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
        })
    }
}
impl ::treesitter_types::Spanned for LazyField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftSection<'tree> {
    pub span: ::treesitter_types::Span,
    pub left_operand: Expression<'tree>,
    pub operator: LeftSectionOperator<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LeftSection<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "left_section");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left_operand: {
                let child = node
                    .child_by_field_name("left_operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "left_operand",
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
                <LeftSectionOperator as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LeftSection<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let<'tree> {
    pub span: ::treesitter_types::Span,
    pub binds: ::core::option::Option<LocalBinds<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Let<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            binds: match node.child_by_field_name("binds") {
                Some(child) => {
                    Some(
                        <LocalBinds as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Let<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetIn<'tree> {
    pub span: ::treesitter_types::Span,
    pub binds: ::core::option::Option<LocalBinds<'tree>>,
    pub expression: LetInExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetIn<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_in");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            binds: match node.child_by_field_name("binds") {
                Some(child) => {
                    Some(
                        <LocalBinds as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <LetInExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetIn<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinearFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub arrow: LinearFunctionArrow,
    pub multiplicity: ::core::option::Option<Modifier<'tree>>,
    pub parameter: LinearFunctionParameter<'tree>,
    pub result: QuantifiedType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinearFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "linear_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arrow: {
                let child = node
                    .child_by_field_name("arrow")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "arrow",
                        node,
                    ))?;
                <LinearFunctionArrow as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            multiplicity: match node.child_by_field_name("multiplicity") {
                Some(child) => {
                    Some(
                        <Modifier as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            parameter: {
                let child = node
                    .child_by_field_name("parameter")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "parameter",
                        node,
                    ))?;
                <LinearFunctionParameter as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            result: {
                let child = node
                    .child_by_field_name("result")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "result",
                        node,
                    ))?;
                <QuantifiedType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LinearFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: ::std::vec::Vec<ListElement<'tree>>,
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
            element: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("element", &mut cursor) {
                    items
                        .push(
                            <ListElement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for List<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListComprehension<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ListComprehensionExpression<'tree>,
    pub qualifiers: ::std::vec::Vec<Qualifiers<'tree>>,
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
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <ListComprehensionExpression as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            qualifiers: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("qualifiers", &mut cursor) {
                    items
                        .push(
                            <Qualifiers as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ListComprehension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: LiteralChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Literal<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "literal");
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
                                        <LiteralChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <LiteralChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <LiteralChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Literal<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalBinds<'tree> {
    pub span: ::treesitter_types::Span,
    pub decl: ::std::vec::Vec<LocalBindsDecl<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalBinds<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_binds");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            decl: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("decl", &mut cursor) {
                    items
                        .push(
                            <LocalBindsDecl as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LocalBinds<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: MatchExpression<'tree>,
    pub guards: ::core::option::Option<Guards<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Match<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <MatchExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            guards: match node.child_by_field_name("guards") {
                Some(child) => {
                    Some(
                        <Guards as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Match<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Modifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "modifier");
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
                                if !cursor2.goto_next_sibling() {
                                    break;
                                }
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
impl ::treesitter_types::Spanned for Modifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModuleId<'tree>>,
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
                            <ModuleId as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Module<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleExport<'tree> {
    pub span: ::treesitter_types::Span,
    pub module: Module<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleExport<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_export");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "module",
                        node,
                    ))?;
                <Module as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleExport<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiWayIf<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#match: ::std::vec::Vec<Match<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MultiWayIf<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "multi_way_if");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#match: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("match", &mut cursor) {
                    items
                        .push(
                            <Match as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for MultiWayIf<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Namespace<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Namespace<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Namespace<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Negation<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ::core::option::Option<Expression<'tree>>,
    pub minus: ::core::option::Option<NegationMinus>,
    pub number: ::core::option::Option<NegationNumber<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Negation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "negation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: match node.child_by_field_name("expression") {
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
            minus: match node.child_by_field_name("minus") {
                Some(child) => {
                    Some(
                        <NegationMinus as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            number: match node.child_by_field_name("number") {
                Some(child) => {
                    Some(
                        <NegationNumber as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Negation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Newtype<'tree> {
    pub span: ::treesitter_types::Span,
    pub constructor: ::core::option::Option<NewtypeConstructor<'tree>>,
    pub constructors: ::core::option::Option<GadtConstructors<'tree>>,
    pub context: ::core::option::Option<Context<'tree>>,
    pub deriving: ::std::vec::Vec<Deriving<'tree>>,
    pub forall: ::core::option::Option<NewtypeForall<'tree>>,
    pub kind: ::core::option::Option<QuantifiedType<'tree>>,
    pub name: ::core::option::Option<NewtypeName<'tree>>,
    pub patterns: ::core::option::Option<NewtypePatterns<'tree>>,
    pub children: ::core::option::Option<NewtypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Newtype<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "newtype");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constructor: match node.child_by_field_name("constructor") {
                Some(child) => {
                    Some(
                        <NewtypeConstructor as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            constructors: match node.child_by_field_name("constructors") {
                Some(child) => {
                    Some(
                        <GadtConstructors as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            context: match node.child_by_field_name("context") {
                Some(child) => {
                    Some(
                        <Context as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            deriving: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("deriving", &mut cursor) {
                    items
                        .push(
                            <Deriving as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <NewtypeForall as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            kind: match node.child_by_field_name("kind") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <NewtypeName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <NewtypePatterns as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <NewtypeChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Newtype<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewtypeConstructor<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: NewtypeConstructorField<'tree>,
    pub name: NewtypeConstructorName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewtypeConstructor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "newtype_constructor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "field",
                        node,
                    ))?;
                <NewtypeConstructorField as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <NewtypeConstructorName as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NewtypeConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Operator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Operator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Operator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Operator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parens<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ::core::option::Option<ParensExpression<'tree>>,
    pub kind: ::core::option::Option<QuantifiedType<'tree>>,
    pub name: ::core::option::Option<ParensName<'tree>>,
    pub pattern: ::core::option::Option<ParensPattern<'tree>>,
    pub patterns: ::core::option::Option<ParensPatterns<'tree>>,
    pub r#type: ::core::option::Option<ParensType<'tree>>,
    pub children: ::core::option::Option<ParensChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Parens<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parens");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: match node.child_by_field_name("expression") {
                Some(child) => {
                    Some(
                        <ParensExpression as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            kind: match node.child_by_field_name("kind") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <ParensName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => {
                    Some(
                        <ParensPattern as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <ParensPatterns as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <ParensType as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <ParensChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Parens<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternGuard<'tree> {
    pub span: ::treesitter_types::Span,
    pub arrow: PatternGuardArrow,
    pub expression: PatternGuardExpression<'tree>,
    pub pattern: PatternGuardPattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternGuard<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pattern_guard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arrow: {
                let child = node
                    .child_by_field_name("arrow")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "arrow",
                        node,
                    ))?;
                <PatternGuardArrow as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <PatternGuardExpression as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            pattern: {
                let child = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "pattern",
                        node,
                    ))?;
                <PatternGuardPattern as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PatternGuard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternSynonym<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PatternSynonymChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternSynonym<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pattern_synonym");
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
                                        <PatternSynonymChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <PatternSynonymChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <PatternSynonymChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PatternSynonym<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Patterns<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PatternsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Patterns<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "patterns");
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
                            <PatternsChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Patterns<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefix<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: ::std::vec::Vec<PrefixField<'tree>>,
    pub name: ::core::option::Option<PrefixName<'tree>>,
    pub r#type: ::core::option::Option<QuantifiedType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Prefix<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("field", &mut cursor) {
                    items
                        .push(
                            <PrefixField as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <PrefixName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Prefix<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixId<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PrefixIdChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixId<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_id");
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
                                        <PrefixIdChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <PrefixIdChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <PrefixIdChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PrefixId<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixList<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixList<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrefixList<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrefixList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixTuple<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixTuple<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_tuple");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrefixTuple<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrefixTuple<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixUnboxedSum<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixUnboxedSum<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_unboxed_sum");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrefixUnboxedSum<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrefixUnboxedSum<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixUnboxedTuple<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixUnboxedTuple<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_unboxed_tuple");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrefixUnboxedTuple<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrefixUnboxedTuple<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Projection<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: Expression<'tree>,
    pub field: FieldName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Projection<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "projection");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "field",
                        node,
                    ))?;
                <FieldName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Projection<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: ::std::vec::Vec<Variable<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProjectionSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "projection_selector");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("field", &mut cursor) {
                    items
                        .push(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ProjectionSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Promoted<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PromotedChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Promoted<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "promoted");
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
                                        <PromotedChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <PromotedChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <PromotedChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Promoted<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qualified<'tree> {
    pub span: ::treesitter_types::Span,
    pub id: QualifiedId<'tree>,
    pub module: Module<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Qualified<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "qualified");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            id: {
                let child = node
                    .child_by_field_name("id")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "id",
                        node,
                    ))?;
                <QualifiedId as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "module",
                        node,
                    ))?;
                <Module as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Qualified<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qualifiers<'tree> {
    pub span: ::treesitter_types::Span,
    pub qualifier: ::std::vec::Vec<Qualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Qualifiers<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "qualifiers");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            qualifier: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("qualifier", &mut cursor) {
                    items
                        .push(
                            <Qualifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Qualifiers<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantifiedVariables<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<QuantifiedVariablesChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuantifiedVariables<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quantified_variables");
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
                            <QuantifiedVariablesChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QuantifiedVariables<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quasiquote<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<QuasiquoteBody<'tree>>,
    pub quoter: Quoter<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Quasiquote<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quasiquote");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(
                        <QuasiquoteBody as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            quoter: {
                let child = node
                    .child_by_field_name("quoter")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "quoter",
                        node,
                    ))?;
                <Quoter as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Quasiquote<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote<'tree> {
    pub span: ::treesitter_types::Span,
    pub quoter: ::core::option::Option<QuoteQuoter>,
    pub children: ::core::option::Option<QuoteChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Quote<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quote");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            quoter: match node.child_by_field_name("quoter") {
                Some(child) => {
                    Some(
                        <QuoteQuoter as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <QuoteChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Quote<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedDecls<'tree> {
    pub span: ::treesitter_types::Span,
    pub declaration: ::std::vec::Vec<Declaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedDecls<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_decls");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declaration: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declaration", &mut cursor) {
                    items
                        .push(
                            <Declaration as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QuotedDecls<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: QuotedExpressionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_expression");
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
                                        <QuotedExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <QuotedExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <QuotedExpressionChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: QuotedPatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_pattern");
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
                                        <QuotedPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <QuotedPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <QuotedPatternChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: QuotedTypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_type");
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
                                        <QuotedTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <QuotedTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <QuotedTypeChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quoter<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: QuoterChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Quoter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoter");
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
                                        <QuoterChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <QuoterChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <QuoterChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Quoter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rec<'tree> {
    pub span: ::treesitter_types::Span,
    pub statement: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Rec<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rec");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            statement: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("statement", &mut cursor) {
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
impl ::treesitter_types::Spanned for Rec<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'tree> {
    pub span: ::treesitter_types::Span,
    pub arrow: ::std::vec::Vec<RecordArrow>,
    pub constructor: ::core::option::Option<Pattern<'tree>>,
    pub expression: ::core::option::Option<Expression<'tree>>,
    pub field: ::std::vec::Vec<RecordField<'tree>>,
    pub fields: ::core::option::Option<Fields<'tree>>,
    pub name: ::core::option::Option<Constructor<'tree>>,
    pub r#type: ::core::option::Option<QuantifiedType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Record<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arrow: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("arrow", &mut cursor) {
                    items
                        .push(
                            <RecordArrow as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            constructor: match node.child_by_field_name("constructor") {
                Some(child) => {
                    Some(
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            expression: match node.child_by_field_name("expression") {
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
            field: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("field", &mut cursor) {
                    items
                        .push(
                            <RecordField as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            fields: match node.child_by_field_name("fields") {
                Some(child) => {
                    Some(
                        <Fields as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <Constructor as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Record<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightSection<'tree> {
    pub span: ::treesitter_types::Span,
    pub right_operand: Expression<'tree>,
    pub children: RightSectionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RightSection<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "right_section");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            right_operand: {
                let child = node
                    .child_by_field_name("right_operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "right_operand",
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
                                        <RightSectionChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <RightSectionChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <RightSectionChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RightSection<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleAnnotation<'tree> {
    pub span: ::treesitter_types::Span,
    pub role: ::std::vec::Vec<TypeRole<'tree>>,
    pub r#type: RoleAnnotationType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RoleAnnotation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "role_annotation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            role: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("role", &mut cursor) {
                    items
                        .push(
                            <TypeRole as ::treesitter_types::FromNode>::from_node(
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
                <RoleAnnotationType as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RoleAnnotation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: ::core::option::Option<Constraints<'tree>>,
    pub expression: ::core::option::Option<Expression<'tree>>,
    pub kind: ::core::option::Option<QuantifiedType<'tree>>,
    pub name: ::core::option::Option<SignatureName<'tree>>,
    pub names: ::core::option::Option<BindingList<'tree>>,
    pub pattern: ::core::option::Option<Pattern<'tree>>,
    pub synonym: ::core::option::Option<SignatureSynonym<'tree>>,
    pub r#type: ::core::option::Option<QuantifiedType<'tree>>,
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
            constraint: match node.child_by_field_name("constraint") {
                Some(child) => {
                    Some(
                        <Constraints as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            expression: match node.child_by_field_name("expression") {
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
            kind: match node.child_by_field_name("kind") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <SignatureName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            names: match node.child_by_field_name("names") {
                Some(child) => {
                    Some(
                        <BindingList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => {
                    Some(
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
            synonym: match node.child_by_field_name("synonym") {
                Some(child) => {
                    Some(
                        <SignatureSynonym as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Signature<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Special<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: SpecialChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Special<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "special");
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
                                        <SpecialChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <SpecialChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <SpecialChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Special<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Splice<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: SpliceExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Splice<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splice");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Splice<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Star<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Star<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "star");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Star<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Star<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Strict<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Strict<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "strict");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "pattern",
                        node,
                    ))?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Strict<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrictField<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StrictField<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "strict_field");
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
        })
    }
}
impl ::treesitter_types::Spanned for StrictField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThQuotedName<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<ThQuotedNameName<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThQuotedName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "th_quoted_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <ThQuotedNameName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(<Type as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ThQuotedName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopSplice<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TopSplice<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "top_splice");
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
impl ::treesitter_types::Spanned for TopSplice<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: ::core::option::Option<TransformKey<'tree>>,
    pub transformation: TransformTransformation<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Transform<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "transform");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: match node.child_by_field_name("key") {
                Some(child) => {
                    Some(
                        <TransformKey as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            transformation: {
                let child = node
                    .child_by_field_name("transformation")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "transformation",
                        node,
                    ))?;
                <TransformTransformation as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Transform<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: ::std::vec::Vec<TupleElement<'tree>>,
    pub children: ::std::vec::Vec<Constraints<'tree>>,
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
            element: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("element", &mut cursor) {
                    items
                        .push(
                            <TupleElement as ::treesitter_types::FromNode>::from_node(
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items
                        .push(
                            <Constraints as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Tuple<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeApplication<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeApplication<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_application");
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
        })
    }
}
impl ::treesitter_types::Spanned for TypeApplication<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeBinder<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeBinder<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_binder");
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
        })
    }
}
impl ::treesitter_types::Spanned for TypeBinder<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFamily<'tree> {
    pub span: ::treesitter_types::Span,
    pub closed_family: ::core::option::Option<TypeFamilyClosedFamily<'tree>>,
    pub kind: ::core::option::Option<QuantifiedType<'tree>>,
    pub name: ::core::option::Option<TypeFamilyName<'tree>>,
    pub patterns: ::core::option::Option<TypeParams<'tree>>,
    pub children: ::std::vec::Vec<TypeFamilyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeFamily<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_family");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            closed_family: match node.child_by_field_name("closed_family") {
                Some(child) => {
                    Some(
                        <TypeFamilyClosedFamily as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            kind: match node.child_by_field_name("kind") {
                Some(child) => {
                    Some(
                        <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <TypeFamilyName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypeParams as ::treesitter_types::FromNode>::from_node(
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
                            <TypeFamilyChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeFamily<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFamilyInjectivity<'tree> {
    pub span: ::treesitter_types::Span,
    pub determined: ::std::vec::Vec<Variable<'tree>>,
    pub result: Variable<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeFamilyInjectivity<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_family_injectivity");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            determined: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("determined", &mut cursor) {
                    items
                        .push(
                            <Variable as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            result: {
                let child = node
                    .child_by_field_name("result")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "result",
                        node,
                    ))?;
                <Variable as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeFamilyInjectivity<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFamilyResult<'tree> {
    pub span: ::treesitter_types::Span,
    pub result: QuantifiedType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeFamilyResult<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_family_result");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            result: {
                let child = node
                    .child_by_field_name("result")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "result",
                        node,
                    ))?;
                <QuantifiedType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeFamilyResult<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeInstance<'tree> {
    pub span: ::treesitter_types::Span,
    pub forall: ::core::option::Option<TypeInstanceForall<'tree>>,
    pub name: ::core::option::Option<TypeInstanceName<'tree>>,
    pub patterns: ::core::option::Option<TypePatterns<'tree>>,
    pub children: ::std::vec::Vec<TypeInstanceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeInstance<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_instance");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            forall: match node.child_by_field_name("forall") {
                Some(child) => {
                    Some(
                        <TypeInstanceForall as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <TypeInstanceName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypePatterns as ::treesitter_types::FromNode>::from_node(
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
                            <TypeInstanceChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeInstance<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParams<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeParam<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParams<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_params");
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
                            <TypeParam as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeParams<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypePatterns<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypePatternsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypePatterns<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_patterns");
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
                            <TypePatternsChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypePatterns<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeRole<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeRole<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_role");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TypeRole<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TypeRole<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeSynomym<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<TypeSynomymName<'tree>>,
    pub patterns: ::core::option::Option<TypeParams<'tree>>,
    pub r#type: TypeSynomymType<'tree>,
    pub children: ::core::option::Option<TypeSynomymChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSynomym<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_synomym");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <TypeSynomymName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            patterns: match node.child_by_field_name("patterns") {
                Some(child) => {
                    Some(
                        <TypeParams as ::treesitter_types::FromNode>::from_node(
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
                <TypeSynomymType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <TypeSynomymChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeSynomym<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedQuote<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<QuotedExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedQuote<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typed_quote");
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
                            <QuotedExpression as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypedQuote<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnboxedSum<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: ::std::vec::Vec<UnboxedSumElement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnboxedSum<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unboxed_sum");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            element: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("element", &mut cursor) {
                    items
                        .push(
                            <UnboxedSumElement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for UnboxedSum<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnboxedTuple<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: ::std::vec::Vec<UnboxedTupleElement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnboxedTuple<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unboxed_tuple");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            element: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("element", &mut cursor) {
                    items
                        .push(
                            <UnboxedTupleElement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for UnboxedTuple<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnboxedUnit<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnboxedUnit<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unboxed_unit");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UnboxedUnit<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UnboxedUnit<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Via<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: QuantifiedType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Via<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "via");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <QuantifiedType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Via<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: ViewPatternExpression<'tree>,
    pub pattern: ViewPatternPattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ViewPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "view_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node
                    .child_by_field_name("expression")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "expression",
                        node,
                    ))?;
                <ViewPatternExpression as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            pattern: {
                let child = node
                    .child_by_field_name("pattern")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "pattern",
                        node,
                    ))?;
                <ViewPatternPattern as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ViewPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllNames<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AllNames<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "all_names");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AllNames<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AllNames<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallingConvention<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallingConvention<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "calling_convention");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for CallingConvention<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for CallingConvention<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Char<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Char<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "char");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Char<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Char<'_> {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constructor<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Constructor<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Constructor<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Constructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cpp<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Cpp<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "cpp");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Cpp<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Cpp<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Haddock<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Haddock<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "haddock");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Haddock<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Haddock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplicitVariable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitVariable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_variable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ImplicitVariable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ImplicitVariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportPackage<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportPackage<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_package");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ImportPackage<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ImportPackage<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Label<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "label");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Label<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Label<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleId<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleId<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_id");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ModuleId<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ModuleId<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pragma<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pragma<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pragma");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Pragma<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Pragma<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuasiquoteBody<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuasiquoteBody<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quasiquote_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for QuasiquoteBody<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for QuasiquoteBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Safety<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Safety<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "safety");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Safety<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Safety<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for String<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for String<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for String<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Variable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Variable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Variable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlternativePattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlternativePattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for AlternativePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyArgument<'tree> {
    ExplicitType(::std::boxed::Box<ExplicitType<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    KindApplication(::std::boxed::Box<KindApplication<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypeApplication(::std::boxed::Box<TypeApplication<'tree>>),
    TypeBinder(::std::boxed::Box<TypeBinder<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ApplyArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "explicit_type" => {
                Ok(
                    Self::ExplicitType(
                        ::std::boxed::Box::new(
                            <ExplicitType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "kind_application" => {
                Ok(
                    Self::KindApplication(
                        ::std::boxed::Box::new(
                            <KindApplication as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_application" => {
                Ok(
                    Self::TypeApplication(
                        ::std::boxed::Box::new(
                            <TypeApplication as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_binder" => {
                Ok(
                    Self::TypeBinder(
                        ::std::boxed::Box::new(
                            <TypeBinder as ::treesitter_types::FromNode>::from_node(
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
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(
                            node,
                            src,
                        ) {
                            Ok(Self::Type(::std::boxed::Box::new(v)))
                        } else {
                            Err(
                                ::treesitter_types::ParseError::unexpected_kind(
                                    _other,
                                    node,
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ApplyArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExplicitType(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::KindApplication(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeApplication(inner) => inner.span(),
            Self::TypeBinder(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyConstructor<'tree> {
    Constraint(::std::boxed::Box<Constraint<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ApplyConstructor<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Constraint as ::treesitter_types::FromNode>::from_node(
            node,
            src,
        ) {
            Ok(Self::Constraint(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Type(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(node.kind(), node))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ApplyConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constraint(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplyFunction<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ApplyFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(
            node,
            src,
        ) {
            Ok(Self::Expression(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                node,
                src,
            ) {
                Ok(Self::Pattern(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(node.kind(), node))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ApplyFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticSequenceFrom<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArithmeticSequenceFrom<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArithmeticSequenceFrom<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticSequenceStep<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArithmeticSequenceStep<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArithmeticSequenceStep<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticSequenceTo<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArithmeticSequenceTo<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArithmeticSequenceTo<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssociatedTypeNamespace {
    Type(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssociatedTypeNamespace {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type" => Ok(Self::Type(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssociatedTypeNamespace {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssociatedTypeType<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssociatedTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AssociatedTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindArrow {
    LArrow(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindArrow {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "<-" => Ok(Self::LArrow(::treesitter_types::Span::from(node))),
            "←" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BindArrow {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LArrow(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BindExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindName<'tree> {
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BindName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrefixId(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for BindPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingListName<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindingListName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BindingListName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BooleanChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BooleanChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BooleanChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for CaseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChildrenElement<'tree> {
    AllNames(::std::boxed::Box<AllNames<'tree>>),
    AssociatedType(::std::boxed::Box<AssociatedType<'tree>>),
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ChildrenElement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "all_names" => {
                Ok(
                    Self::AllNames(
                        ::std::boxed::Box::new(
                            <AllNames as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "associated_type" => {
                Ok(
                    Self::AssociatedType(
                        ::std::boxed::Box::new(
                            <AssociatedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ChildrenElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AllNames(inner) => inner.span(),
            Self::AssociatedType(inner) => inner.span(),
            Self::Constructor(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ClassChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionalElse<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalElse<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConditionalElse<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionalIf<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalIf<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConditionalIf<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionalThen<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalThen<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConditionalThen<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructorSynonymName<'tree> {
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorSynonymName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConstructorSynonymName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrefixId(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructorSynonymPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorSynonymPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorSynonymPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextArrow {
    FatArrow(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContextArrow {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "=>" => Ok(Self::FatArrow(::treesitter_types::Span::from(node))),
            "⇒" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ContextArrow {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FatArrow(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataConstructorConstructor<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Prefix(::std::boxed::Box<Prefix<'tree>>),
    Record(::std::boxed::Box<Record<'tree>>),
    Special(::std::boxed::Box<Special<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataConstructorConstructor<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix" => {
                Ok(
                    Self::Prefix(
                        ::std::boxed::Box::new(
                            <Prefix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "record" => {
                Ok(
                    Self::Record(
                        ::std::boxed::Box::new(
                            <Record as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "special" => {
                Ok(
                    Self::Special(
                        ::std::boxed::Box::new(
                            <Special as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataConstructorConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Prefix(inner) => inner.span(),
            Self::Record(inner) => inner.span(),
            Self::Special(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataConstructorForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataConstructorForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataConstructorForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataFamilyName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataFamilyName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DataFamilyName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataFamilyChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataFamilyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataFamilyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataInstanceChildren<'tree> {
    DataType(::std::boxed::Box<DataType<'tree>>),
    Newtype(::std::boxed::Box<Newtype<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataInstanceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "data_type" => {
                Ok(
                    Self::DataType(
                        ::std::boxed::Box::new(
                            <DataType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "newtype" => {
                Ok(
                    Self::Newtype(
                        ::std::boxed::Box::new(
                            <Newtype as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataInstanceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DataType(inner) => inner.span(),
            Self::Newtype(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTypeConstructors<'tree> {
    DataConstructors(::std::boxed::Box<DataConstructors<'tree>>),
    GadtConstructors(::std::boxed::Box<GadtConstructors<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataTypeConstructors<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "data_constructors" => {
                Ok(
                    Self::DataConstructors(
                        ::std::boxed::Box::new(
                            <DataConstructors as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "gadt_constructors" => {
                Ok(
                    Self::GadtConstructors(
                        ::std::boxed::Box::new(
                            <GadtConstructors as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataTypeConstructors<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DataConstructors(inner) => inner.span(),
            Self::GadtConstructors(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTypeForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataTypeForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataTypeForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTypeName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataTypeName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DataTypeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTypePatterns<'tree> {
    TypeParams(::std::boxed::Box<TypeParams<'tree>>),
    TypePatterns(::std::boxed::Box<TypePatterns<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataTypePatterns<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_params" => {
                Ok(
                    Self::TypeParams(
                        ::std::boxed::Box::new(
                            <TypeParams as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_patterns" => {
                Ok(
                    Self::TypePatterns(
                        ::std::boxed::Box::new(
                            <TypePatterns as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataTypePatterns<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TypeParams(inner) => inner.span(),
            Self::TypePatterns(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataTypeChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DataTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DataTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationsChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Import(::std::boxed::Box<Import<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "import" => {
                Ok(
                    Self::Import(
                        ::std::boxed::Box::new(
                            <Import as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Declaration as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Declaration(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Import(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefaultTypesType<'tree> {
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultTypesType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for DefaultTypesType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivingInstanceForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DerivingInstanceForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DerivingInstanceForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivingInstanceName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DerivingInstanceName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DerivingInstanceName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivingInstanceChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DerivingInstanceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DerivingInstanceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoModuleId {
    Do(::treesitter_types::Span),
    Mdo(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoModuleId {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "do" => Ok(Self::Do(::treesitter_types::Span::from(node))),
            "mdo" => Ok(Self::Mdo(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DoModuleId {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Do(span) => *span,
            Self::Mdo(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquationForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EquationForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for EquationForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquationName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EquationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for EquationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquationPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EquationPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for EquationPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquationChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EquationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for EquationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ExpChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportType<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ExportType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportVariable<'tree> {
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportVariable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ExportVariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Qualified(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldParameter<'tree> {
    LazyField(::std::boxed::Box<LazyField<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    StrictField(::std::boxed::Box<StrictField<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldParameter<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lazy_field" => {
                Ok(
                    Self::LazyField(
                        ::std::boxed::Box::new(
                            <LazyField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict_field" => {
                Ok(
                    Self::StrictField(
                        ::std::boxed::Box::new(
                            <StrictField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FieldParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LazyField(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType<'tree> {
    LazyField(::std::boxed::Box<LazyField<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    StrictField(::std::boxed::Box<StrictField<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lazy_field" => {
                Ok(
                    Self::LazyField(
                        ::std::boxed::Box::new(
                            <LazyField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict_field" => {
                Ok(
                    Self::StrictField(
                        ::std::boxed::Box::new(
                            <StrictField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FieldType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LazyField(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldPathField<'tree> {
    FieldName(::std::boxed::Box<FieldName<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPathField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_name" => {
                Ok(
                    Self::FieldName(
                        ::std::boxed::Box::new(
                            <FieldName as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldPathField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldName(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldPatternField<'tree> {
    FieldName(::std::boxed::Box<FieldName<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPatternField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_name" => {
                Ok(
                    Self::FieldName(
                        ::std::boxed::Box::new(
                            <FieldName as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldPatternField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldName(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldPatternPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
    ViewPattern(::std::boxed::Box<ViewPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "view_pattern" => {
                Ok(
                    Self::ViewPattern(
                        ::std::boxed::Box::new(
                            <ViewPattern as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FieldPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldUpdateExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldUpdateExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldUpdateExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldUpdateField<'tree> {
    FieldName(::std::boxed::Box<FieldName<'tree>>),
    FieldPath(::std::boxed::Box<FieldPath<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldUpdateField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_name" => {
                Ok(
                    Self::FieldName(
                        ::std::boxed::Box::new(
                            <FieldName as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "field_path" => {
                Ok(
                    Self::FieldPath(
                        ::std::boxed::Box::new(
                            <FieldPath as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldUpdateField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldName(inner) => inner.span(),
            Self::FieldPath(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixityAssociativity {
    Infix(::treesitter_types::Span),
    Infixl(::treesitter_types::Span),
    Infixr(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FixityAssociativity {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => Ok(Self::Infix(::treesitter_types::Span::from(node))),
            "infixl" => Ok(Self::Infixl(::treesitter_types::Span::from(node))),
            "infixr" => Ok(Self::Infixr(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FixityAssociativity {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(span) => *span,
            Self::Infixl(span) => *span,
            Self::Infixr(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixityOperator<'tree> {
    Comma(::treesitter_types::Span),
    ConstructorOperator(::std::boxed::Box<ConstructorOperator<'tree>>),
    InfixId(::std::boxed::Box<InfixId<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FixityOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "constructor_operator" => {
                Ok(
                    Self::ConstructorOperator(
                        ::std::boxed::Box::new(
                            <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix_id" => {
                Ok(
                    Self::InfixId(
                        ::std::boxed::Box::new(
                            <InfixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "operator" => {
                Ok(
                    Self::Operator(
                        ::std::boxed::Box::new(
                            <Operator as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FixityOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::ConstructorOperator(inner) => inner.span(),
            Self::InfixId(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForallQuantifier {
    Forall(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForallQuantifier {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => Ok(Self::Forall(::treesitter_types::Span::from(node))),
            "∀" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForallQuantifier {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForallRequiredQuantifier {
    Forall(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForallRequiredQuantifier {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => Ok(Self::Forall(::treesitter_types::Span::from(node))),
            "∀" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForallRequiredQuantifier {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionArrow {
    Arrow(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionArrow {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "->" => Ok(Self::Arrow(::treesitter_types::Span::from(node))),
            "→" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionArrow {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arrow(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionName<'tree> {
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FunctionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrefixId(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionParameter<'tree> {
    LazyField(::std::boxed::Box<LazyField<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    StrictField(::std::boxed::Box<StrictField<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionParameter<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lazy_field" => {
                Ok(
                    Self::LazyField(
                        ::std::boxed::Box::new(
                            <LazyField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict_field" => {
                Ok(
                    Self::StrictField(
                        ::std::boxed::Box::new(
                            <StrictField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FunctionParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LazyField(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionHeadParensName<'tree> {
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionHeadParensName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FunctionHeadParensName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrefixId(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GadtConstructorForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GadtConstructorForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GadtConstructorForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GadtConstructorName<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GadtConstructorName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GadtConstructorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GadtConstructorType<'tree> {
    Prefix(::std::boxed::Box<Prefix<'tree>>),
    Record(::std::boxed::Box<Record<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GadtConstructorType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "prefix" => {
                Ok(
                    Self::Prefix(
                        ::std::boxed::Box::new(
                            <Prefix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "record" => {
                Ok(
                    Self::Record(
                        ::std::boxed::Box::new(
                            <Record as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GadtConstructorType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Prefix(inner) => inner.span(),
            Self::Record(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorArrow {
    LArrow(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GeneratorArrow {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "<-" => Ok(Self::LArrow(::treesitter_types::Span::from(node))),
            "←" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GeneratorArrow {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LArrow(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GeneratorExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GeneratorExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GeneratorPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for GeneratorPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupClassifier<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GroupClassifier<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GroupClassifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupKey<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GroupKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GroupKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportNameType<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportNameType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportNameType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportNameVariable<'tree> {
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportNameVariable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportNameVariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Qualified(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferredChildren<'tree> {
    Annotated(::std::boxed::Box<Annotated<'tree>>),
    TypeParam(::std::boxed::Box<TypeParam<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InferredChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated" => {
                Ok(
                    Self::Annotated(
                        ::std::boxed::Box::new(
                            <Annotated as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <TypeParam as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::TypeParam(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for InferredChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotated(inner) => inner.span(),
            Self::TypeParam(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixLeftOperand<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LazyField(::std::boxed::Box<LazyField<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    StrictField(::std::boxed::Box<StrictField<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypeParam(::std::boxed::Box<TypeParam<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixLeftOperand<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lazy_field" => {
                Ok(
                    Self::LazyField(
                        ::std::boxed::Box::new(
                            <LazyField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict_field" => {
                Ok(
                    Self::StrictField(
                        ::std::boxed::Box::new(
                            <StrictField as ::treesitter_types::FromNode>::from_node(
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
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(
                            node,
                            src,
                        ) {
                            Ok(Self::Type(::std::boxed::Box::new(v)))
                        } else {
                            if let Ok(v) = <TypeParam as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            ) {
                                Ok(Self::TypeParam(::std::boxed::Box::new(v)))
                            } else {
                                Err(
                                    ::treesitter_types::ParseError::unexpected_kind(
                                        _other,
                                        node,
                                    ),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for InfixLeftOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LazyField(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeParam(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixOperator<'tree> {
    ConstructorOperator(::std::boxed::Box<ConstructorOperator<'tree>>),
    InfixId(::std::boxed::Box<InfixId<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    Promoted(::std::boxed::Box<Promoted<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor_operator" => {
                Ok(
                    Self::ConstructorOperator(
                        ::std::boxed::Box::new(
                            <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix_id" => {
                Ok(
                    Self::InfixId(
                        ::std::boxed::Box::new(
                            <InfixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "operator" => {
                Ok(
                    Self::Operator(
                        ::std::boxed::Box::new(
                            <Operator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "promoted" => {
                Ok(
                    Self::Promoted(
                        ::std::boxed::Box::new(
                            <Promoted as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InfixOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstructorOperator(inner) => inner.span(),
            Self::InfixId(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::Promoted(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixRightOperand<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LazyField(::std::boxed::Box<LazyField<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    StrictField(::std::boxed::Box<StrictField<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypeParam(::std::boxed::Box<TypeParam<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixRightOperand<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lazy_field" => {
                Ok(
                    Self::LazyField(
                        ::std::boxed::Box::new(
                            <LazyField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict_field" => {
                Ok(
                    Self::StrictField(
                        ::std::boxed::Box::new(
                            <StrictField as ::treesitter_types::FromNode>::from_node(
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
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(
                            node,
                            src,
                        ) {
                            Ok(Self::Type(::std::boxed::Box::new(v)))
                        } else {
                            if let Ok(v) = <TypeParam as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            ) {
                                Ok(Self::TypeParam(::std::boxed::Box::new(v)))
                            } else {
                                Err(
                                    ::treesitter_types::ParseError::unexpected_kind(
                                        _other,
                                        node,
                                    ),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for InfixRightOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LazyField(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeParam(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixIdChildren<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixIdChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InfixIdChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InstanceForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InstanceName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InstanceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KindSignatureName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KindSignatureName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for KindSignatureName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KindSignatureChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KindSignatureChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for KindSignatureChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LambdaExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LambdaExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeftSectionOperator<'tree> {
    ConstructorOperator(::std::boxed::Box<ConstructorOperator<'tree>>),
    InfixId(::std::boxed::Box<InfixId<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LeftSectionOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor_operator" => {
                Ok(
                    Self::ConstructorOperator(
                        ::std::boxed::Box::new(
                            <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix_id" => {
                Ok(
                    Self::InfixId(
                        ::std::boxed::Box::new(
                            <InfixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "operator" => {
                Ok(
                    Self::Operator(
                        ::std::boxed::Box::new(
                            <Operator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LeftSectionOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstructorOperator(inner) => inner.span(),
            Self::InfixId(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LetInExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetInExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LetInExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinearFunctionArrow {
    Arrow(::treesitter_types::Span),
    MinusGtDot(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinearFunctionArrow {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "->" => Ok(Self::Arrow(::treesitter_types::Span::from(node))),
            "->." => Ok(Self::MinusGtDot(::treesitter_types::Span::from(node))),
            "→" | "⊸" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LinearFunctionArrow {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arrow(span) => *span,
            Self::MinusGtDot(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinearFunctionParameter<'tree> {
    LazyField(::std::boxed::Box<LazyField<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    StrictField(::std::boxed::Box<StrictField<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinearFunctionParameter<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lazy_field" => {
                Ok(
                    Self::LazyField(
                        ::std::boxed::Box::new(
                            <LazyField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict_field" => {
                Ok(
                    Self::StrictField(
                        ::std::boxed::Box::new(
                            <StrictField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for LinearFunctionParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LazyField(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListElement<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
    ViewPattern(::std::boxed::Box<ViewPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListElement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "view_pattern" => {
                Ok(
                    Self::ViewPattern(
                        ::std::boxed::Box::new(
                            <ViewPattern as ::treesitter_types::FromNode>::from_node(
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
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            node,
                            src,
                        ) {
                            Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                        } else {
                            Err(
                                ::treesitter_types::ParseError::unexpected_kind(
                                    _other,
                                    node,
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ListElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListComprehensionExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListComprehensionExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ListComprehensionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralChildren<'tree> {
    Char(::std::boxed::Box<Char<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "char" => {
                Ok(
                    Self::Char(
                        ::std::boxed::Box::new(
                            <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "float" => {
                Ok(
                    Self::Float(
                        ::std::boxed::Box::new(
                            <Float as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "integer" => {
                Ok(
                    Self::Integer(
                        ::std::boxed::Box::new(
                            <Integer as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "string" => {
                Ok(
                    Self::String(
                        ::std::boxed::Box::new(
                            <String as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Char(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalBindsDecl<'tree> {
    Decl(::std::boxed::Box<Decl<'tree>>),
    Fixity(::std::boxed::Box<Fixity<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalBindsDecl<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "fixity" => {
                Ok(
                    Self::Fixity(
                        ::std::boxed::Box::new(
                            <Fixity as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Decl as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Decl(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for LocalBindsDecl<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Decl(inner) => inner.span(),
            Self::Fixity(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for MatchExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NegationMinus {
    Minus(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegationMinus {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NegationMinus {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Minus(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NegationNumber<'tree> {
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegationNumber<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "float" => {
                Ok(
                    Self::Float(
                        ::std::boxed::Box::new(
                            <Float as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "integer" => {
                Ok(
                    Self::Integer(
                        ::std::boxed::Box::new(
                            <Integer as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NegationNumber<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewtypeForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewtypeForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NewtypeForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewtypeName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewtypeName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NewtypeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewtypePatterns<'tree> {
    TypeParams(::std::boxed::Box<TypeParams<'tree>>),
    TypePatterns(::std::boxed::Box<TypePatterns<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewtypePatterns<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_params" => {
                Ok(
                    Self::TypeParams(
                        ::std::boxed::Box::new(
                            <TypeParams as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_patterns" => {
                Ok(
                    Self::TypePatterns(
                        ::std::boxed::Box::new(
                            <TypePatterns as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NewtypePatterns<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TypeParams(inner) => inner.span(),
            Self::TypePatterns(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewtypeChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewtypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NewtypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewtypeConstructorField<'tree> {
    Field(::std::boxed::Box<Field<'tree>>),
    Record(::std::boxed::Box<Record<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewtypeConstructorField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field" => {
                Ok(
                    Self::Field(
                        ::std::boxed::Box::new(
                            <Field as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "record" => {
                Ok(
                    Self::Record(
                        ::std::boxed::Box::new(
                            <Record as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NewtypeConstructorField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Field(inner) => inner.span(),
            Self::Record(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewtypeConstructorName<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewtypeConstructorName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NewtypeConstructorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParensExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParensExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParensExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParensName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParensName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParensName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParensPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
    ViewPattern(::std::boxed::Box<ViewPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParensPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "view_pattern" => {
                Ok(
                    Self::ViewPattern(
                        ::std::boxed::Box::new(
                            <ViewPattern as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ParensPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParensPatterns<'tree> {
    TypeParams(::std::boxed::Box<TypeParams<'tree>>),
    TypePatterns(::std::boxed::Box<TypePatterns<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParensPatterns<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_params" => {
                Ok(
                    Self::TypeParams(
                        ::std::boxed::Box::new(
                            <TypeParams as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_patterns" => {
                Ok(
                    Self::TypePatterns(
                        ::std::boxed::Box::new(
                            <TypePatterns as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParensPatterns<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TypeParams(inner) => inner.span(),
            Self::TypePatterns(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParensType<'tree> {
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParensType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ParensType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParensChildren<'tree> {
    Annotated(::std::boxed::Box<Annotated<'tree>>),
    Constraints(::std::boxed::Box<Constraints<'tree>>),
    Infix(::std::boxed::Box<Infix<'tree>>),
    TypeParam(::std::boxed::Box<TypeParam<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParensChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated" => {
                Ok(
                    Self::Annotated(
                        ::std::boxed::Box::new(
                            <Annotated as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Constraints as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Constraints(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = <TypeParam as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::TypeParam(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for ParensChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotated(inner) => inner.span(),
            Self::Constraints(inner) => inner.span(),
            Self::Infix(inner) => inner.span(),
            Self::TypeParam(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternGuardArrow {
    LArrow(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternGuardArrow {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "<-" => Ok(Self::LArrow(::treesitter_types::Span::from(node))),
            "←" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PatternGuardArrow {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LArrow(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternGuardExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternGuardExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PatternGuardExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternGuardPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternGuardPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PatternGuardPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternSynonymChildren<'tree> {
    Equation(::std::boxed::Box<Equation<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternSynonymChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "equation" => {
                Ok(
                    Self::Equation(
                        ::std::boxed::Box::new(
                            <Equation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PatternSynonymChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Equation(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternsChildren<'tree> {
    ExplicitType(::std::boxed::Box<ExplicitType<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    TypeBinder(::std::boxed::Box<TypeBinder<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "explicit_type" => {
                Ok(
                    Self::ExplicitType(
                        ::std::boxed::Box::new(
                            <ExplicitType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_binder" => {
                Ok(
                    Self::TypeBinder(
                        ::std::boxed::Box::new(
                            <TypeBinder as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PatternsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExplicitType(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::TypeBinder(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixField<'tree> {
    LazyField(::std::boxed::Box<LazyField<'tree>>),
    StrictField(::std::boxed::Box<StrictField<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lazy_field" => {
                Ok(
                    Self::LazyField(
                        ::std::boxed::Box::new(
                            <LazyField as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "strict_field" => {
                Ok(
                    Self::StrictField(
                        ::std::boxed::Box::new(
                            <StrictField as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PrefixField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LazyField(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixName<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PrefixName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixIdChildren<'tree> {
    ConstructorOperator(::std::boxed::Box<ConstructorOperator<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixIdChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor_operator" => {
                Ok(
                    Self::ConstructorOperator(
                        ::std::boxed::Box::new(
                            <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "operator" => {
                Ok(
                    Self::Operator(
                        ::std::boxed::Box::new(
                            <Operator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PrefixIdChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstructorOperator(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromotedChildren<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    ConstructorOperator(::std::boxed::Box<ConstructorOperator<'tree>>),
    EmptyList(::std::boxed::Box<EmptyList<'tree>>),
    InfixId(::std::boxed::Box<InfixId<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixTuple(::std::boxed::Box<PrefixTuple<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PromotedChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "constructor_operator" => {
                Ok(
                    Self::ConstructorOperator(
                        ::std::boxed::Box::new(
                            <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "empty_list" => {
                Ok(
                    Self::EmptyList(
                        ::std::boxed::Box::new(
                            <EmptyList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix_id" => {
                Ok(
                    Self::InfixId(
                        ::std::boxed::Box::new(
                            <InfixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list" => {
                Ok(
                    Self::List(
                        ::std::boxed::Box::new(
                            <List as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "operator" => {
                Ok(
                    Self::Operator(
                        ::std::boxed::Box::new(
                            <Operator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_tuple" => {
                Ok(
                    Self::PrefixTuple(
                        ::std::boxed::Box::new(
                            <PrefixTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tuple" => {
                Ok(
                    Self::Tuple(
                        ::std::boxed::Box::new(
                            <Tuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PromotedChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::ConstructorOperator(inner) => inner.span(),
            Self::EmptyList(inner) => inner.span(),
            Self::InfixId(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixTuple(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualifiedId<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    ConstructorOperator(::std::boxed::Box<ConstructorOperator<'tree>>),
    FieldName(::std::boxed::Box<FieldName<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedId<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "constructor_operator" => {
                Ok(
                    Self::ConstructorOperator(
                        ::std::boxed::Box::new(
                            <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "field_name" => {
                Ok(
                    Self::FieldName(
                        ::std::boxed::Box::new(
                            <FieldName as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "operator" => {
                Ok(
                    Self::Operator(
                        ::std::boxed::Box::new(
                            <Operator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QualifiedId<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::ConstructorOperator(inner) => inner.span(),
            Self::FieldName(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantifiedVariablesChildren<'tree> {
    Inferred(::std::boxed::Box<Inferred<'tree>>),
    TypeParam(::std::boxed::Box<TypeParam<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuantifiedVariablesChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "inferred" => {
                Ok(
                    Self::Inferred(
                        ::std::boxed::Box::new(
                            <Inferred as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <TypeParam as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::TypeParam(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for QuantifiedVariablesChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Inferred(inner) => inner.span(),
            Self::TypeParam(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuoteQuoter {
    D(::treesitter_types::Span),
    E(::treesitter_types::Span),
    P(::treesitter_types::Span),
    T(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuoteQuoter {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "d" => Ok(Self::D(::treesitter_types::Span::from(node))),
            "e" => Ok(Self::E(::treesitter_types::Span::from(node))),
            "p" => Ok(Self::P(::treesitter_types::Span::from(node))),
            "t" => Ok(Self::T(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuoteQuoter {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::D(span) => *span,
            Self::E(span) => *span,
            Self::P(span) => *span,
            Self::T(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuoteChildren<'tree> {
    QuotedDecls(::std::boxed::Box<QuotedDecls<'tree>>),
    QuotedExpression(::std::boxed::Box<QuotedExpression<'tree>>),
    QuotedPattern(::std::boxed::Box<QuotedPattern<'tree>>),
    QuotedType(::std::boxed::Box<QuotedType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuoteChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "quoted_decls" => {
                Ok(
                    Self::QuotedDecls(
                        ::std::boxed::Box::new(
                            <QuotedDecls as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quoted_expression" => {
                Ok(
                    Self::QuotedExpression(
                        ::std::boxed::Box::new(
                            <QuotedExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quoted_pattern" => {
                Ok(
                    Self::QuotedPattern(
                        ::std::boxed::Box::new(
                            <QuotedPattern as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quoted_type" => {
                Ok(
                    Self::QuotedType(
                        ::std::boxed::Box::new(
                            <QuotedType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QuoteChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QuotedDecls(inner) => inner.span(),
            Self::QuotedExpression(inner) => inner.span(),
            Self::QuotedPattern(inner) => inner.span(),
            Self::QuotedType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QuotedExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedPatternChildren<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for QuotedPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedTypeChildren<'tree> {
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for QuotedTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuoterChildren<'tree> {
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuoterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QuoterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Qualified(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordArrow {
    Arrow(::treesitter_types::Span),
    X(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordArrow {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "->" => Ok(Self::Arrow(::treesitter_types::Span::from(node))),
            "→" => Ok(Self::X(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RecordArrow {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arrow(span) => *span,
            Self::X(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordField<'tree> {
    Field(::std::boxed::Box<Field<'tree>>),
    FieldPattern(::std::boxed::Box<FieldPattern<'tree>>),
    FieldUpdate(::std::boxed::Box<FieldUpdate<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field" => {
                Ok(
                    Self::Field(
                        ::std::boxed::Box::new(
                            <Field as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "field_pattern" => {
                Ok(
                    Self::FieldPattern(
                        ::std::boxed::Box::new(
                            <FieldPattern as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "field_update" => {
                Ok(
                    Self::FieldUpdate(
                        ::std::boxed::Box::new(
                            <FieldUpdate as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for RecordField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Field(inner) => inner.span(),
            Self::FieldPattern(inner) => inner.span(),
            Self::FieldUpdate(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RightSectionChildren<'tree> {
    ConstructorOperator(::std::boxed::Box<ConstructorOperator<'tree>>),
    InfixId(::std::boxed::Box<InfixId<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RightSectionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor_operator" => {
                Ok(
                    Self::ConstructorOperator(
                        ::std::boxed::Box::new(
                            <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "infix_id" => {
                Ok(
                    Self::InfixId(
                        ::std::boxed::Box::new(
                            <InfixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "operator" => {
                Ok(
                    Self::Operator(
                        ::std::boxed::Box::new(
                            <Operator as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for RightSectionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstructorOperator(inner) => inner.span(),
            Self::InfixId(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleAnnotationType<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RoleAnnotationType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for RoleAnnotationType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureName<'tree> {
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SignatureName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SignatureName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrefixId(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureSynonym<'tree> {
    BindingList(::std::boxed::Box<BindingList<'tree>>),
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SignatureSynonym<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binding_list" => {
                Ok(
                    Self::BindingList(
                        ::std::boxed::Box::new(
                            <BindingList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SignatureSynonym<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BindingList(inner) => inner.span(),
            Self::Constructor(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialChildren<'tree> {
    EmptyList(::std::boxed::Box<EmptyList<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnboxedSum(::std::boxed::Box<UnboxedSum<'tree>>),
    UnboxedTuple(::std::boxed::Box<UnboxedTuple<'tree>>),
    UnboxedUnit(::std::boxed::Box<UnboxedUnit<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpecialChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_list" => {
                Ok(
                    Self::EmptyList(
                        ::std::boxed::Box::new(
                            <EmptyList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tuple" => {
                Ok(
                    Self::Tuple(
                        ::std::boxed::Box::new(
                            <Tuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_sum" => {
                Ok(
                    Self::UnboxedSum(
                        ::std::boxed::Box::new(
                            <UnboxedSum as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_tuple" => {
                Ok(
                    Self::UnboxedTuple(
                        ::std::boxed::Box::new(
                            <UnboxedTuple as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unboxed_unit" => {
                Ok(
                    Self::UnboxedUnit(
                        ::std::boxed::Box::new(
                            <UnboxedUnit as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SpecialChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EmptyList(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnboxedSum(inner) => inner.span(),
            Self::UnboxedTuple(inner) => inner.span(),
            Self::UnboxedUnit(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpliceExpression<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    ImplicitVariable(::std::boxed::Box<ImplicitVariable<'tree>>),
    Label(::std::boxed::Box<Label<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpliceExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "implicit_variable" => {
                Ok(
                    Self::ImplicitVariable(
                        ::std::boxed::Box::new(
                            <ImplicitVariable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "label" => {
                Ok(
                    Self::Label(
                        ::std::boxed::Box::new(
                            <Label as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "literal" => {
                Ok(
                    Self::Literal(
                        ::std::boxed::Box::new(
                            <Literal as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SpliceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::ImplicitVariable(inner) => inner.span(),
            Self::Label(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThQuotedNameName<'tree> {
    Constructor(::std::boxed::Box<Constructor<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThQuotedNameName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constructor" => {
                Ok(
                    Self::Constructor(
                        ::std::boxed::Box::new(
                            <Constructor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variable" => {
                Ok(
                    Self::Variable(
                        ::std::boxed::Box::new(
                            <Variable as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ThQuotedNameName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constructor(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformKey<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TransformKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TransformKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformTransformation<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TransformTransformation<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TransformTransformation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TupleElement<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
    ViewPattern(::std::boxed::Box<ViewPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleElement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "view_pattern" => {
                Ok(
                    Self::ViewPattern(
                        ::std::boxed::Box::new(
                            <ViewPattern as ::treesitter_types::FromNode>::from_node(
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
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            node,
                            src,
                        ) {
                            Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                        } else {
                            Err(
                                ::treesitter_types::ParseError::unexpected_kind(
                                    _other,
                                    node,
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for TupleElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeFamilyClosedFamily<'tree> {
    AbstractFamily(::std::boxed::Box<AbstractFamily<'tree>>),
    Equations(::std::boxed::Box<Equations<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeFamilyClosedFamily<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_family" => {
                Ok(
                    Self::AbstractFamily(
                        ::std::boxed::Box::new(
                            <AbstractFamily as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "equations" => {
                Ok(
                    Self::Equations(
                        ::std::boxed::Box::new(
                            <Equations as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeFamilyClosedFamily<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractFamily(inner) => inner.span(),
            Self::Equations(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeFamilyName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeFamilyName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeFamilyName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeFamilyChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    TypeFamilyInjectivity(::std::boxed::Box<TypeFamilyInjectivity<'tree>>),
    TypeFamilyResult(::std::boxed::Box<TypeFamilyResult<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeFamilyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_family_injectivity" => {
                Ok(
                    Self::TypeFamilyInjectivity(
                        ::std::boxed::Box::new(
                            <TypeFamilyInjectivity as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_family_result" => {
                Ok(
                    Self::TypeFamilyResult(
                        ::std::boxed::Box::new(
                            <TypeFamilyResult as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeFamilyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::TypeFamilyInjectivity(inner) => inner.span(),
            Self::TypeFamilyResult(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeInstanceForall<'tree> {
    Forall(::std::boxed::Box<Forall<'tree>>),
    ForallRequired(::std::boxed::Box<ForallRequired<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeInstanceForall<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "forall" => {
                Ok(
                    Self::Forall(
                        ::std::boxed::Box::new(
                            <Forall as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "forall_required" => {
                Ok(
                    Self::ForallRequired(
                        ::std::boxed::Box::new(
                            <ForallRequired as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeInstanceForall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeInstanceName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    Qualified(::std::boxed::Box<Qualified<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeInstanceName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified" => {
                Ok(
                    Self::Qualified(
                        ::std::boxed::Box::new(
                            <Qualified as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeInstanceName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeInstanceChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeInstanceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for TypeInstanceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypePatternsChildren<'tree> {
    KindApplication(::std::boxed::Box<KindApplication<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypePatternsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "kind_application" => {
                Ok(
                    Self::KindApplication(
                        ::std::boxed::Box::new(
                            <KindApplication as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypePatternsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::KindApplication(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSynomymName<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    PrefixId(::std::boxed::Box<PrefixId<'tree>>),
    PrefixList(::std::boxed::Box<PrefixList<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSynomymName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => {
                Ok(
                    Self::Name(
                        ::std::boxed::Box::new(
                            <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "prefix_id" => {
                Ok(
                    Self::PrefixId(
                        ::std::boxed::Box::new(
                            <PrefixId as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "prefix_list" => {
                Ok(
                    Self::PrefixList(
                        ::std::boxed::Box::new(
                            <PrefixList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unit" => {
                Ok(
                    Self::Unit(
                        ::std::boxed::Box::new(
                            <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeSynomymName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSynomymType<'tree> {
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSynomymType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for TypeSynomymType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSynomymChildren<'tree> {
    Infix(::std::boxed::Box<Infix<'tree>>),
    Parens(::std::boxed::Box<Parens<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSynomymChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "infix" => {
                Ok(
                    Self::Infix(
                        ::std::boxed::Box::new(
                            <Infix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parens" => {
                Ok(
                    Self::Parens(
                        ::std::boxed::Box::new(
                            <Parens as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeSynomymChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Infix(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnboxedSumElement<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
    ViewPattern(::std::boxed::Box<ViewPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnboxedSumElement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "view_pattern" => {
                Ok(
                    Self::ViewPattern(
                        ::std::boxed::Box::new(
                            <ViewPattern as ::treesitter_types::FromNode>::from_node(
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
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            node,
                            src,
                        ) {
                            Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                        } else {
                            Err(
                                ::treesitter_types::ParseError::unexpected_kind(
                                    _other,
                                    node,
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for UnboxedSumElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnboxedTupleElement<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    QuantifiedType(::std::boxed::Box<QuantifiedType<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
    ViewPattern(::std::boxed::Box<ViewPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnboxedTupleElement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "view_pattern" => {
                Ok(
                    Self::ViewPattern(
                        ::std::boxed::Box::new(
                            <ViewPattern as ::treesitter_types::FromNode>::from_node(
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
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = <QuantifiedType as ::treesitter_types::FromNode>::from_node(
                            node,
                            src,
                        ) {
                            Ok(Self::QuantifiedType(::std::boxed::Box::new(v)))
                        } else {
                            Err(
                                ::treesitter_types::ParseError::unexpected_kind(
                                    _other,
                                    node,
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for UnboxedTupleElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewPatternExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ViewPatternExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ViewPatternExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewPatternPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Signature(::std::boxed::Box<Signature<'tree>>),
    ViewPattern(::std::boxed::Box<ViewPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ViewPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "signature" => {
                Ok(
                    Self::Signature(
                        ::std::boxed::Box::new(
                            <Signature as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "view_pattern" => {
                Ok(
                    Self::ViewPattern(
                        ::std::boxed::Box::new(
                            <ViewPattern as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ViewPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    ClassDecl(ClassDecl<'tree>),
    Constraint(Constraint<'tree>),
    Constraints(Constraints<'tree>),
    Decl(Decl<'tree>),
    Declaration(Declaration<'tree>),
    Expression(Expression<'tree>),
    Guard(Guard<'tree>),
    InstanceDecl(InstanceDecl<'tree>),
    Pattern(Pattern<'tree>),
    Qualifier(Qualifier<'tree>),
    QuantifiedType(QuantifiedType<'tree>),
    Statement(Statement<'tree>),
    Type(Type<'tree>),
    TypeParam(TypeParam<'tree>),
    AbstractFamily(AbstractFamily<'tree>),
    Alternative(Alternative<'tree>),
    Alternatives(Alternatives<'tree>),
    Annotated(Annotated<'tree>),
    Apply(Apply<'tree>),
    ArithmeticSequence(ArithmeticSequence<'tree>),
    As(As<'tree>),
    AssociatedType(AssociatedType<'tree>),
    Bind(Bind<'tree>),
    BindingList(BindingList<'tree>),
    Boolean(Boolean<'tree>),
    Case(Case<'tree>),
    Children(Children<'tree>),
    Class(Class<'tree>),
    ClassDeclarations(ClassDeclarations<'tree>),
    Conditional(Conditional<'tree>),
    ConstructorOperator(ConstructorOperator<'tree>),
    ConstructorSynonym(ConstructorSynonym<'tree>),
    ConstructorSynonyms(ConstructorSynonyms<'tree>),
    Context(Context<'tree>),
    DataConstructor(DataConstructor<'tree>),
    DataConstructors(DataConstructors<'tree>),
    DataFamily(DataFamily<'tree>),
    DataInstance(DataInstance<'tree>),
    DataType(DataType<'tree>),
    Declarations(Declarations<'tree>),
    DefaultSignature(DefaultSignature<'tree>),
    DefaultTypes(DefaultTypes<'tree>),
    Deriving(Deriving<'tree>),
    DerivingInstance(DerivingInstance<'tree>),
    DerivingStrategy(DerivingStrategy<'tree>),
    Do(Do<'tree>),
    DoModule(DoModule<'tree>),
    EmptyList(EmptyList<'tree>),
    Entity(Entity<'tree>),
    Equation(Equation<'tree>),
    Equations(Equations<'tree>),
    Exp(Exp<'tree>),
    ExplicitType(ExplicitType<'tree>),
    Export(Export<'tree>),
    Exports(Exports<'tree>),
    Field(Field<'tree>),
    FieldName(FieldName<'tree>),
    FieldPath(FieldPath<'tree>),
    FieldPattern(FieldPattern<'tree>),
    FieldUpdate(FieldUpdate<'tree>),
    Fields(Fields<'tree>),
    Fixity(Fixity<'tree>),
    Forall(Forall<'tree>),
    ForallRequired(ForallRequired<'tree>),
    ForeignExport(ForeignExport<'tree>),
    ForeignImport(ForeignImport<'tree>),
    Function(Function<'tree>),
    FunctionHeadParens(FunctionHeadParens<'tree>),
    Fundep(Fundep<'tree>),
    Fundeps(Fundeps<'tree>),
    GadtConstructor(GadtConstructor<'tree>),
    GadtConstructors(GadtConstructors<'tree>),
    Generator(Generator<'tree>),
    Group(Group<'tree>),
    Guards(Guards<'tree>),
    Haskell(Haskell<'tree>),
    Header(Header<'tree>),
    ImplicitParameter(ImplicitParameter<'tree>),
    Import(Import<'tree>),
    ImportList(ImportList<'tree>),
    ImportName(ImportName<'tree>),
    Imports(Imports<'tree>),
    Inferred(Inferred<'tree>),
    Infix(Infix<'tree>),
    InfixId(InfixId<'tree>),
    Instance(Instance<'tree>),
    InstanceDeclarations(InstanceDeclarations<'tree>),
    Integer(Integer<'tree>),
    Invisible(Invisible<'tree>),
    Irrefutable(Irrefutable<'tree>),
    KindApplication(KindApplication<'tree>),
    KindSignature(KindSignature<'tree>),
    Lambda(Lambda<'tree>),
    LambdaCase(LambdaCase<'tree>),
    LambdaCases(LambdaCases<'tree>),
    LazyField(LazyField<'tree>),
    LeftSection(LeftSection<'tree>),
    Let(Let<'tree>),
    LetIn(LetIn<'tree>),
    LinearFunction(LinearFunction<'tree>),
    List(List<'tree>),
    ListComprehension(ListComprehension<'tree>),
    Literal(Literal<'tree>),
    LocalBinds(LocalBinds<'tree>),
    Match(Match<'tree>),
    Modifier(Modifier<'tree>),
    Module(Module<'tree>),
    ModuleExport(ModuleExport<'tree>),
    MultiWayIf(MultiWayIf<'tree>),
    Namespace(Namespace<'tree>),
    Negation(Negation<'tree>),
    Newtype(Newtype<'tree>),
    NewtypeConstructor(NewtypeConstructor<'tree>),
    Operator(Operator<'tree>),
    Parens(Parens<'tree>),
    PatternGuard(PatternGuard<'tree>),
    PatternSynonym(PatternSynonym<'tree>),
    Patterns(Patterns<'tree>),
    Prefix(Prefix<'tree>),
    PrefixId(PrefixId<'tree>),
    PrefixList(PrefixList<'tree>),
    PrefixTuple(PrefixTuple<'tree>),
    PrefixUnboxedSum(PrefixUnboxedSum<'tree>),
    PrefixUnboxedTuple(PrefixUnboxedTuple<'tree>),
    Projection(Projection<'tree>),
    ProjectionSelector(ProjectionSelector<'tree>),
    Promoted(Promoted<'tree>),
    Qualified(Qualified<'tree>),
    Qualifiers(Qualifiers<'tree>),
    QuantifiedVariables(QuantifiedVariables<'tree>),
    Quasiquote(Quasiquote<'tree>),
    Quote(Quote<'tree>),
    QuotedDecls(QuotedDecls<'tree>),
    QuotedExpression(QuotedExpression<'tree>),
    QuotedPattern(QuotedPattern<'tree>),
    QuotedType(QuotedType<'tree>),
    Quoter(Quoter<'tree>),
    Rec(Rec<'tree>),
    Record(Record<'tree>),
    RightSection(RightSection<'tree>),
    RoleAnnotation(RoleAnnotation<'tree>),
    Signature(Signature<'tree>),
    Special(Special<'tree>),
    Splice(Splice<'tree>),
    Star(Star<'tree>),
    Strict(Strict<'tree>),
    StrictField(StrictField<'tree>),
    ThQuotedName(ThQuotedName<'tree>),
    TopSplice(TopSplice<'tree>),
    Transform(Transform<'tree>),
    Tuple(Tuple<'tree>),
    TypeApplication(TypeApplication<'tree>),
    TypeBinder(TypeBinder<'tree>),
    TypeFamily(TypeFamily<'tree>),
    TypeFamilyInjectivity(TypeFamilyInjectivity<'tree>),
    TypeFamilyResult(TypeFamilyResult<'tree>),
    TypeInstance(TypeInstance<'tree>),
    TypeParams(TypeParams<'tree>),
    TypePatterns(TypePatterns<'tree>),
    TypeRole(TypeRole<'tree>),
    TypeSynomym(TypeSynomym<'tree>),
    TypedQuote(TypedQuote<'tree>),
    UnboxedSum(UnboxedSum<'tree>),
    UnboxedTuple(UnboxedTuple<'tree>),
    UnboxedUnit(UnboxedUnit<'tree>),
    Unit(Unit<'tree>),
    Via(Via<'tree>),
    ViewPattern(ViewPattern<'tree>),
    Wildcard(Wildcard<'tree>),
    AllNames(AllNames<'tree>),
    CallingConvention(CallingConvention<'tree>),
    Char(Char<'tree>),
    Comment(Comment<'tree>),
    Constructor(Constructor<'tree>),
    Cpp(Cpp<'tree>),
    Float(Float<'tree>),
    Haddock(Haddock<'tree>),
    ImplicitVariable(ImplicitVariable<'tree>),
    ImportPackage(ImportPackage<'tree>),
    Label(Label<'tree>),
    ModuleId(ModuleId<'tree>),
    Name(Name<'tree>),
    Pragma(Pragma<'tree>),
    QuasiquoteBody(QuasiquoteBody<'tree>),
    Safety(Safety<'tree>),
    String(String<'tree>),
    Variable(Variable<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "class_decl" => {
                <ClassDecl as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDecl)
                    .unwrap_or(Self::Unknown(node))
            }
            "constraint" => {
                <Constraint as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Constraint)
                    .unwrap_or(Self::Unknown(node))
            }
            "constraints" => {
                <Constraints as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Constraints)
                    .unwrap_or(Self::Unknown(node))
            }
            "decl" => {
                <Decl as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Decl)
                    .unwrap_or(Self::Unknown(node))
            }
            "declaration" => {
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Declaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression" => {
                <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Expression)
                    .unwrap_or(Self::Unknown(node))
            }
            "guard" => {
                <Guard as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Guard)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance_decl" => {
                <InstanceDecl as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InstanceDecl)
                    .unwrap_or(Self::Unknown(node))
            }
            "pattern" => {
                <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Pattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "qualifier" => {
                <Qualifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Qualifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "quantified_type" => {
                <QuantifiedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuantifiedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "statement" => {
                <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Statement)
                    .unwrap_or(Self::Unknown(node))
            }
            "type" => {
                <Type as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Type)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_param" => {
                <TypeParam as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParam)
                    .unwrap_or(Self::Unknown(node))
            }
            "abstract_family" => {
                <AbstractFamily as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AbstractFamily)
                    .unwrap_or(Self::Unknown(node))
            }
            "alternative" => {
                <Alternative as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Alternative)
                    .unwrap_or(Self::Unknown(node))
            }
            "alternatives" => {
                <Alternatives as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Alternatives)
                    .unwrap_or(Self::Unknown(node))
            }
            "annotated" => {
                <Annotated as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Annotated)
                    .unwrap_or(Self::Unknown(node))
            }
            "apply" => {
                <Apply as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Apply)
                    .unwrap_or(Self::Unknown(node))
            }
            "arithmetic_sequence" => {
                <ArithmeticSequence as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ArithmeticSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "as" => {
                <As as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::As)
                    .unwrap_or(Self::Unknown(node))
            }
            "associated_type" => {
                <AssociatedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssociatedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "bind" => {
                <Bind as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Bind)
                    .unwrap_or(Self::Unknown(node))
            }
            "binding_list" => {
                <BindingList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BindingList)
                    .unwrap_or(Self::Unknown(node))
            }
            "boolean" => {
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Boolean)
                    .unwrap_or(Self::Unknown(node))
            }
            "case" => {
                <Case as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Case)
                    .unwrap_or(Self::Unknown(node))
            }
            "children" => {
                <Children as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Children)
                    .unwrap_or(Self::Unknown(node))
            }
            "class" => {
                <Class as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Class)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_declarations" => {
                <ClassDeclarations as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDeclarations)
                    .unwrap_or(Self::Unknown(node))
            }
            "conditional" => {
                <Conditional as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Conditional)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_operator" => {
                <ConstructorOperator as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ConstructorOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_synonym" => {
                <ConstructorSynonym as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ConstructorSynonym)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_synonyms" => {
                <ConstructorSynonyms as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ConstructorSynonyms)
                    .unwrap_or(Self::Unknown(node))
            }
            "context" => {
                <Context as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Context)
                    .unwrap_or(Self::Unknown(node))
            }
            "data_constructor" => {
                <DataConstructor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DataConstructor)
                    .unwrap_or(Self::Unknown(node))
            }
            "data_constructors" => {
                <DataConstructors as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DataConstructors)
                    .unwrap_or(Self::Unknown(node))
            }
            "data_family" => {
                <DataFamily as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DataFamily)
                    .unwrap_or(Self::Unknown(node))
            }
            "data_instance" => {
                <DataInstance as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DataInstance)
                    .unwrap_or(Self::Unknown(node))
            }
            "data_type" => {
                <DataType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DataType)
                    .unwrap_or(Self::Unknown(node))
            }
            "declarations" => {
                <Declarations as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Declarations)
                    .unwrap_or(Self::Unknown(node))
            }
            "default_signature" => {
                <DefaultSignature as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DefaultSignature)
                    .unwrap_or(Self::Unknown(node))
            }
            "default_types" => {
                <DefaultTypes as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DefaultTypes)
                    .unwrap_or(Self::Unknown(node))
            }
            "deriving" => {
                <Deriving as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Deriving)
                    .unwrap_or(Self::Unknown(node))
            }
            "deriving_instance" => {
                <DerivingInstance as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DerivingInstance)
                    .unwrap_or(Self::Unknown(node))
            }
            "deriving_strategy" => {
                <DerivingStrategy as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DerivingStrategy)
                    .unwrap_or(Self::Unknown(node))
            }
            "do" => {
                <Do as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Do)
                    .unwrap_or(Self::Unknown(node))
            }
            "do_module" => {
                <DoModule as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DoModule)
                    .unwrap_or(Self::Unknown(node))
            }
            "empty_list" => {
                <EmptyList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EmptyList)
                    .unwrap_or(Self::Unknown(node))
            }
            "entity" => {
                <Entity as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Entity)
                    .unwrap_or(Self::Unknown(node))
            }
            "equation" => {
                <Equation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Equation)
                    .unwrap_or(Self::Unknown(node))
            }
            "equations" => {
                <Equations as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Equations)
                    .unwrap_or(Self::Unknown(node))
            }
            "exp" => {
                <Exp as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Exp)
                    .unwrap_or(Self::Unknown(node))
            }
            "explicit_type" => {
                <ExplicitType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExplicitType)
                    .unwrap_or(Self::Unknown(node))
            }
            "export" => {
                <Export as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Export)
                    .unwrap_or(Self::Unknown(node))
            }
            "exports" => {
                <Exports as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Exports)
                    .unwrap_or(Self::Unknown(node))
            }
            "field" => {
                <Field as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Field)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_name" => {
                <FieldName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldName)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_path" => {
                <FieldPath as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldPath)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_pattern" => {
                <FieldPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_update" => {
                <FieldUpdate as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldUpdate)
                    .unwrap_or(Self::Unknown(node))
            }
            "fields" => {
                <Fields as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Fields)
                    .unwrap_or(Self::Unknown(node))
            }
            "fixity" => {
                <Fixity as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Fixity)
                    .unwrap_or(Self::Unknown(node))
            }
            "forall" => {
                <Forall as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Forall)
                    .unwrap_or(Self::Unknown(node))
            }
            "forall_required" => {
                <ForallRequired as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForallRequired)
                    .unwrap_or(Self::Unknown(node))
            }
            "foreign_export" => {
                <ForeignExport as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForeignExport)
                    .unwrap_or(Self::Unknown(node))
            }
            "foreign_import" => {
                <ForeignImport as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForeignImport)
                    .unwrap_or(Self::Unknown(node))
            }
            "function" => {
                <Function as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Function)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_head_parens" => {
                <FunctionHeadParens as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::FunctionHeadParens)
                    .unwrap_or(Self::Unknown(node))
            }
            "fundep" => {
                <Fundep as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Fundep)
                    .unwrap_or(Self::Unknown(node))
            }
            "fundeps" => {
                <Fundeps as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Fundeps)
                    .unwrap_or(Self::Unknown(node))
            }
            "gadt_constructor" => {
                <GadtConstructor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GadtConstructor)
                    .unwrap_or(Self::Unknown(node))
            }
            "gadt_constructors" => {
                <GadtConstructors as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GadtConstructors)
                    .unwrap_or(Self::Unknown(node))
            }
            "generator" => {
                <Generator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Generator)
                    .unwrap_or(Self::Unknown(node))
            }
            "group" => {
                <Group as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Group)
                    .unwrap_or(Self::Unknown(node))
            }
            "guards" => {
                <Guards as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Guards)
                    .unwrap_or(Self::Unknown(node))
            }
            "haskell" => {
                <Haskell as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Haskell)
                    .unwrap_or(Self::Unknown(node))
            }
            "header" => {
                <Header as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Header)
                    .unwrap_or(Self::Unknown(node))
            }
            "implicit_parameter" => {
                <ImplicitParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImplicitParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "import" => {
                <Import as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Import)
                    .unwrap_or(Self::Unknown(node))
            }
            "import_list" => {
                <ImportList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportList)
                    .unwrap_or(Self::Unknown(node))
            }
            "import_name" => {
                <ImportName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportName)
                    .unwrap_or(Self::Unknown(node))
            }
            "imports" => {
                <Imports as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Imports)
                    .unwrap_or(Self::Unknown(node))
            }
            "inferred" => {
                <Inferred as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Inferred)
                    .unwrap_or(Self::Unknown(node))
            }
            "infix" => {
                <Infix as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Infix)
                    .unwrap_or(Self::Unknown(node))
            }
            "infix_id" => {
                <InfixId as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InfixId)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance" => {
                <Instance as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Instance)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance_declarations" => {
                <InstanceDeclarations as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::InstanceDeclarations)
                    .unwrap_or(Self::Unknown(node))
            }
            "integer" => {
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Integer)
                    .unwrap_or(Self::Unknown(node))
            }
            "invisible" => {
                <Invisible as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Invisible)
                    .unwrap_or(Self::Unknown(node))
            }
            "irrefutable" => {
                <Irrefutable as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Irrefutable)
                    .unwrap_or(Self::Unknown(node))
            }
            "kind_application" => {
                <KindApplication as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KindApplication)
                    .unwrap_or(Self::Unknown(node))
            }
            "kind_signature" => {
                <KindSignature as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KindSignature)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda" => {
                <Lambda as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Lambda)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_case" => {
                <LambdaCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaCase)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_cases" => {
                <LambdaCases as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaCases)
                    .unwrap_or(Self::Unknown(node))
            }
            "lazy_field" => {
                <LazyField as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LazyField)
                    .unwrap_or(Self::Unknown(node))
            }
            "left_section" => {
                <LeftSection as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LeftSection)
                    .unwrap_or(Self::Unknown(node))
            }
            "let" => {
                <Let as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Let)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_in" => {
                <LetIn as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LetIn)
                    .unwrap_or(Self::Unknown(node))
            }
            "linear_function" => {
                <LinearFunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LinearFunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "list" => {
                <List as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::List)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_comprehension" => {
                <ListComprehension as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListComprehension)
                    .unwrap_or(Self::Unknown(node))
            }
            "literal" => {
                <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Literal)
                    .unwrap_or(Self::Unknown(node))
            }
            "local_binds" => {
                <LocalBinds as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalBinds)
                    .unwrap_or(Self::Unknown(node))
            }
            "match" => {
                <Match as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Match)
                    .unwrap_or(Self::Unknown(node))
            }
            "modifier" => {
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Modifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "module" => {
                <Module as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Module)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_export" => {
                <ModuleExport as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleExport)
                    .unwrap_or(Self::Unknown(node))
            }
            "multi_way_if" => {
                <MultiWayIf as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MultiWayIf)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace" => {
                <Namespace as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Namespace)
                    .unwrap_or(Self::Unknown(node))
            }
            "negation" => {
                <Negation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Negation)
                    .unwrap_or(Self::Unknown(node))
            }
            "newtype" => {
                <Newtype as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Newtype)
                    .unwrap_or(Self::Unknown(node))
            }
            "newtype_constructor" => {
                <NewtypeConstructor as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::NewtypeConstructor)
                    .unwrap_or(Self::Unknown(node))
            }
            "operator" => {
                <Operator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Operator)
                    .unwrap_or(Self::Unknown(node))
            }
            "parens" => {
                <Parens as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Parens)
                    .unwrap_or(Self::Unknown(node))
            }
            "pattern_guard" => {
                <PatternGuard as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PatternGuard)
                    .unwrap_or(Self::Unknown(node))
            }
            "pattern_synonym" => {
                <PatternSynonym as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PatternSynonym)
                    .unwrap_or(Self::Unknown(node))
            }
            "patterns" => {
                <Patterns as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Patterns)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix" => {
                <Prefix as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Prefix)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_id" => {
                <PrefixId as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixId)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_list" => {
                <PrefixList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixList)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_tuple" => {
                <PrefixTuple as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixTuple)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_unboxed_sum" => {
                <PrefixUnboxedSum as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixUnboxedSum)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_unboxed_tuple" => {
                <PrefixUnboxedTuple as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::PrefixUnboxedTuple)
                    .unwrap_or(Self::Unknown(node))
            }
            "projection" => {
                <Projection as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Projection)
                    .unwrap_or(Self::Unknown(node))
            }
            "projection_selector" => {
                <ProjectionSelector as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ProjectionSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "promoted" => {
                <Promoted as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Promoted)
                    .unwrap_or(Self::Unknown(node))
            }
            "qualified" => {
                <Qualified as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Qualified)
                    .unwrap_or(Self::Unknown(node))
            }
            "qualifiers" => {
                <Qualifiers as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Qualifiers)
                    .unwrap_or(Self::Unknown(node))
            }
            "quantified_variables" => {
                <QuantifiedVariables as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::QuantifiedVariables)
                    .unwrap_or(Self::Unknown(node))
            }
            "quasiquote" => {
                <Quasiquote as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Quasiquote)
                    .unwrap_or(Self::Unknown(node))
            }
            "quote" => {
                <Quote as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Quote)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_decls" => {
                <QuotedDecls as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedDecls)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_expression" => {
                <QuotedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_pattern" => {
                <QuotedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_type" => {
                <QuotedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoter" => {
                <Quoter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Quoter)
                    .unwrap_or(Self::Unknown(node))
            }
            "rec" => {
                <Rec as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Rec)
                    .unwrap_or(Self::Unknown(node))
            }
            "record" => {
                <Record as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Record)
                    .unwrap_or(Self::Unknown(node))
            }
            "right_section" => {
                <RightSection as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RightSection)
                    .unwrap_or(Self::Unknown(node))
            }
            "role_annotation" => {
                <RoleAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RoleAnnotation)
                    .unwrap_or(Self::Unknown(node))
            }
            "signature" => {
                <Signature as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Signature)
                    .unwrap_or(Self::Unknown(node))
            }
            "special" => {
                <Special as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Special)
                    .unwrap_or(Self::Unknown(node))
            }
            "splice" => {
                <Splice as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Splice)
                    .unwrap_or(Self::Unknown(node))
            }
            "star" => {
                <Star as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Star)
                    .unwrap_or(Self::Unknown(node))
            }
            "strict" => {
                <Strict as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Strict)
                    .unwrap_or(Self::Unknown(node))
            }
            "strict_field" => {
                <StrictField as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StrictField)
                    .unwrap_or(Self::Unknown(node))
            }
            "th_quoted_name" => {
                <ThQuotedName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThQuotedName)
                    .unwrap_or(Self::Unknown(node))
            }
            "top_splice" => {
                <TopSplice as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TopSplice)
                    .unwrap_or(Self::Unknown(node))
            }
            "transform" => {
                <Transform as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Transform)
                    .unwrap_or(Self::Unknown(node))
            }
            "tuple" => {
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Tuple)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_application" => {
                <TypeApplication as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeApplication)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_binder" => {
                <TypeBinder as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeBinder)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_family" => {
                <TypeFamily as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeFamily)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_family_injectivity" => {
                <TypeFamilyInjectivity as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TypeFamilyInjectivity)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_family_result" => {
                <TypeFamilyResult as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeFamilyResult)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_instance" => {
                <TypeInstance as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeInstance)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_params" => {
                <TypeParams as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParams)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_patterns" => {
                <TypePatterns as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypePatterns)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_role" => {
                <TypeRole as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeRole)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_synomym" => {
                <TypeSynomym as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeSynomym)
                    .unwrap_or(Self::Unknown(node))
            }
            "typed_quote" => {
                <TypedQuote as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypedQuote)
                    .unwrap_or(Self::Unknown(node))
            }
            "unboxed_sum" => {
                <UnboxedSum as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnboxedSum)
                    .unwrap_or(Self::Unknown(node))
            }
            "unboxed_tuple" => {
                <UnboxedTuple as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnboxedTuple)
                    .unwrap_or(Self::Unknown(node))
            }
            "unboxed_unit" => {
                <UnboxedUnit as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnboxedUnit)
                    .unwrap_or(Self::Unknown(node))
            }
            "unit" => {
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Unit)
                    .unwrap_or(Self::Unknown(node))
            }
            "via" => {
                <Via as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Via)
                    .unwrap_or(Self::Unknown(node))
            }
            "view_pattern" => {
                <ViewPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ViewPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "wildcard" => {
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Wildcard)
                    .unwrap_or(Self::Unknown(node))
            }
            "all_names" => {
                <AllNames as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AllNames)
                    .unwrap_or(Self::Unknown(node))
            }
            "calling_convention" => {
                <CallingConvention as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CallingConvention)
                    .unwrap_or(Self::Unknown(node))
            }
            "char" => {
                <Char as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Char)
                    .unwrap_or(Self::Unknown(node))
            }
            "comment" => {
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Comment)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor" => {
                <Constructor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Constructor)
                    .unwrap_or(Self::Unknown(node))
            }
            "cpp" => {
                <Cpp as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Cpp)
                    .unwrap_or(Self::Unknown(node))
            }
            "float" => {
                <Float as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Float)
                    .unwrap_or(Self::Unknown(node))
            }
            "haddock" => {
                <Haddock as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Haddock)
                    .unwrap_or(Self::Unknown(node))
            }
            "implicit_variable" => {
                <ImplicitVariable as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImplicitVariable)
                    .unwrap_or(Self::Unknown(node))
            }
            "import_package" => {
                <ImportPackage as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportPackage)
                    .unwrap_or(Self::Unknown(node))
            }
            "label" => {
                <Label as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Label)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_id" => {
                <ModuleId as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleId)
                    .unwrap_or(Self::Unknown(node))
            }
            "name" => {
                <Name as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Name)
                    .unwrap_or(Self::Unknown(node))
            }
            "pragma" => {
                <Pragma as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Pragma)
                    .unwrap_or(Self::Unknown(node))
            }
            "quasiquote_body" => {
                <QuasiquoteBody as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuasiquoteBody)
                    .unwrap_or(Self::Unknown(node))
            }
            "safety" => {
                <Safety as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Safety)
                    .unwrap_or(Self::Unknown(node))
            }
            "string" => {
                <String as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::String)
                    .unwrap_or(Self::Unknown(node))
            }
            "variable" => {
                <Variable as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Variable)
                    .unwrap_or(Self::Unknown(node))
            }
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDecl(inner) => inner.span(),
            Self::Constraint(inner) => inner.span(),
            Self::Constraints(inner) => inner.span(),
            Self::Decl(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
            Self::InstanceDecl(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Qualifier(inner) => inner.span(),
            Self::QuantifiedType(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeParam(inner) => inner.span(),
            Self::AbstractFamily(inner) => inner.span(),
            Self::Alternative(inner) => inner.span(),
            Self::Alternatives(inner) => inner.span(),
            Self::Annotated(inner) => inner.span(),
            Self::Apply(inner) => inner.span(),
            Self::ArithmeticSequence(inner) => inner.span(),
            Self::As(inner) => inner.span(),
            Self::AssociatedType(inner) => inner.span(),
            Self::Bind(inner) => inner.span(),
            Self::BindingList(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Case(inner) => inner.span(),
            Self::Children(inner) => inner.span(),
            Self::Class(inner) => inner.span(),
            Self::ClassDeclarations(inner) => inner.span(),
            Self::Conditional(inner) => inner.span(),
            Self::ConstructorOperator(inner) => inner.span(),
            Self::ConstructorSynonym(inner) => inner.span(),
            Self::ConstructorSynonyms(inner) => inner.span(),
            Self::Context(inner) => inner.span(),
            Self::DataConstructor(inner) => inner.span(),
            Self::DataConstructors(inner) => inner.span(),
            Self::DataFamily(inner) => inner.span(),
            Self::DataInstance(inner) => inner.span(),
            Self::DataType(inner) => inner.span(),
            Self::Declarations(inner) => inner.span(),
            Self::DefaultSignature(inner) => inner.span(),
            Self::DefaultTypes(inner) => inner.span(),
            Self::Deriving(inner) => inner.span(),
            Self::DerivingInstance(inner) => inner.span(),
            Self::DerivingStrategy(inner) => inner.span(),
            Self::Do(inner) => inner.span(),
            Self::DoModule(inner) => inner.span(),
            Self::EmptyList(inner) => inner.span(),
            Self::Entity(inner) => inner.span(),
            Self::Equation(inner) => inner.span(),
            Self::Equations(inner) => inner.span(),
            Self::Exp(inner) => inner.span(),
            Self::ExplicitType(inner) => inner.span(),
            Self::Export(inner) => inner.span(),
            Self::Exports(inner) => inner.span(),
            Self::Field(inner) => inner.span(),
            Self::FieldName(inner) => inner.span(),
            Self::FieldPath(inner) => inner.span(),
            Self::FieldPattern(inner) => inner.span(),
            Self::FieldUpdate(inner) => inner.span(),
            Self::Fields(inner) => inner.span(),
            Self::Fixity(inner) => inner.span(),
            Self::Forall(inner) => inner.span(),
            Self::ForallRequired(inner) => inner.span(),
            Self::ForeignExport(inner) => inner.span(),
            Self::ForeignImport(inner) => inner.span(),
            Self::Function(inner) => inner.span(),
            Self::FunctionHeadParens(inner) => inner.span(),
            Self::Fundep(inner) => inner.span(),
            Self::Fundeps(inner) => inner.span(),
            Self::GadtConstructor(inner) => inner.span(),
            Self::GadtConstructors(inner) => inner.span(),
            Self::Generator(inner) => inner.span(),
            Self::Group(inner) => inner.span(),
            Self::Guards(inner) => inner.span(),
            Self::Haskell(inner) => inner.span(),
            Self::Header(inner) => inner.span(),
            Self::ImplicitParameter(inner) => inner.span(),
            Self::Import(inner) => inner.span(),
            Self::ImportList(inner) => inner.span(),
            Self::ImportName(inner) => inner.span(),
            Self::Imports(inner) => inner.span(),
            Self::Inferred(inner) => inner.span(),
            Self::Infix(inner) => inner.span(),
            Self::InfixId(inner) => inner.span(),
            Self::Instance(inner) => inner.span(),
            Self::InstanceDeclarations(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Invisible(inner) => inner.span(),
            Self::Irrefutable(inner) => inner.span(),
            Self::KindApplication(inner) => inner.span(),
            Self::KindSignature(inner) => inner.span(),
            Self::Lambda(inner) => inner.span(),
            Self::LambdaCase(inner) => inner.span(),
            Self::LambdaCases(inner) => inner.span(),
            Self::LazyField(inner) => inner.span(),
            Self::LeftSection(inner) => inner.span(),
            Self::Let(inner) => inner.span(),
            Self::LetIn(inner) => inner.span(),
            Self::LinearFunction(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::ListComprehension(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::LocalBinds(inner) => inner.span(),
            Self::Match(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::Module(inner) => inner.span(),
            Self::ModuleExport(inner) => inner.span(),
            Self::MultiWayIf(inner) => inner.span(),
            Self::Namespace(inner) => inner.span(),
            Self::Negation(inner) => inner.span(),
            Self::Newtype(inner) => inner.span(),
            Self::NewtypeConstructor(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::Parens(inner) => inner.span(),
            Self::PatternGuard(inner) => inner.span(),
            Self::PatternSynonym(inner) => inner.span(),
            Self::Patterns(inner) => inner.span(),
            Self::Prefix(inner) => inner.span(),
            Self::PrefixId(inner) => inner.span(),
            Self::PrefixList(inner) => inner.span(),
            Self::PrefixTuple(inner) => inner.span(),
            Self::PrefixUnboxedSum(inner) => inner.span(),
            Self::PrefixUnboxedTuple(inner) => inner.span(),
            Self::Projection(inner) => inner.span(),
            Self::ProjectionSelector(inner) => inner.span(),
            Self::Promoted(inner) => inner.span(),
            Self::Qualified(inner) => inner.span(),
            Self::Qualifiers(inner) => inner.span(),
            Self::QuantifiedVariables(inner) => inner.span(),
            Self::Quasiquote(inner) => inner.span(),
            Self::Quote(inner) => inner.span(),
            Self::QuotedDecls(inner) => inner.span(),
            Self::QuotedExpression(inner) => inner.span(),
            Self::QuotedPattern(inner) => inner.span(),
            Self::QuotedType(inner) => inner.span(),
            Self::Quoter(inner) => inner.span(),
            Self::Rec(inner) => inner.span(),
            Self::Record(inner) => inner.span(),
            Self::RightSection(inner) => inner.span(),
            Self::RoleAnnotation(inner) => inner.span(),
            Self::Signature(inner) => inner.span(),
            Self::Special(inner) => inner.span(),
            Self::Splice(inner) => inner.span(),
            Self::Star(inner) => inner.span(),
            Self::Strict(inner) => inner.span(),
            Self::StrictField(inner) => inner.span(),
            Self::ThQuotedName(inner) => inner.span(),
            Self::TopSplice(inner) => inner.span(),
            Self::Transform(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::TypeApplication(inner) => inner.span(),
            Self::TypeBinder(inner) => inner.span(),
            Self::TypeFamily(inner) => inner.span(),
            Self::TypeFamilyInjectivity(inner) => inner.span(),
            Self::TypeFamilyResult(inner) => inner.span(),
            Self::TypeInstance(inner) => inner.span(),
            Self::TypeParams(inner) => inner.span(),
            Self::TypePatterns(inner) => inner.span(),
            Self::TypeRole(inner) => inner.span(),
            Self::TypeSynomym(inner) => inner.span(),
            Self::TypedQuote(inner) => inner.span(),
            Self::UnboxedSum(inner) => inner.span(),
            Self::UnboxedTuple(inner) => inner.span(),
            Self::UnboxedUnit(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Via(inner) => inner.span(),
            Self::ViewPattern(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
            Self::AllNames(inner) => inner.span(),
            Self::CallingConvention(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::Constructor(inner) => inner.span(),
            Self::Cpp(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Haddock(inner) => inner.span(),
            Self::ImplicitVariable(inner) => inner.span(),
            Self::ImportPackage(inner) => inner.span(),
            Self::Label(inner) => inner.span(),
            Self::ModuleId(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Pragma(inner) => inner.span(),
            Self::QuasiquoteBody(inner) => inner.span(),
            Self::Safety(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
