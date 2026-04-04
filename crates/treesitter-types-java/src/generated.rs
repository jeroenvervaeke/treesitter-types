#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'tree> {
    BinaryIntegerLiteral(::std::boxed::Box<BinaryIntegerLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    DecimalFloatingPointLiteral(::std::boxed::Box<DecimalFloatingPointLiteral<'tree>>),
    DecimalIntegerLiteral(::std::boxed::Box<DecimalIntegerLiteral<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    HexFloatingPointLiteral(::std::boxed::Box<HexFloatingPointLiteral<'tree>>),
    HexIntegerLiteral(::std::boxed::Box<HexIntegerLiteral<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OctalIntegerLiteral(::std::boxed::Box<OctalIntegerLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Literal<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_integer_literal" => Ok(Self::BinaryIntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "decimal_floating_point_literal" => Ok(Self::DecimalFloatingPointLiteral(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DecimalFloatingPointLiteral as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "decimal_integer_literal" => Ok(Self::DecimalIntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DecimalIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <False as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hex_floating_point_literal" => Ok(Self::HexFloatingPointLiteral(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HexFloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "hex_integer_literal" => Ok(Self::HexIntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HexIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "octal_integer_literal" => Ok(Self::OctalIntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OctalIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <True as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Literal<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryIntegerLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::DecimalFloatingPointLiteral(inner) => inner.span(),
            Self::DecimalIntegerLiteral(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::HexFloatingPointLiteral(inner) => inner.span(),
            Self::HexIntegerLiteral(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OctalIntegerLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::True(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleType<'tree> {
    BooleanType(::std::boxed::Box<BooleanType<'tree>>),
    FloatingPointType(::std::boxed::Box<FloatingPointType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    IntegralType(::std::boxed::Box<IntegralType<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    VoidType(::std::boxed::Box<VoidType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean_type" => Ok(Self::BooleanType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BooleanType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "floating_point_type" => Ok(Self::FloatingPointType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FloatingPointType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integral_type" => Ok(Self::IntegralType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IntegralType as ::treesitter_types::FromNode>::from_node(node, src)
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
            "void_type" => Ok(Self::VoidType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VoidType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanType(inner) => inner.span(),
            Self::FloatingPointType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::IntegralType(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::VoidType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'tree> {
    UnannotatedType(::std::boxed::Box<UnannotatedType<'tree>>),
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::UnannotatedType(::std::boxed::Box::new(v)))
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
            Self::UnannotatedType(inner) => inner.span(),
            Self::AnnotatedType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnannotatedType<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnannotatedType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SimpleType as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for UnannotatedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<'tree> {
    AnnotationTypeDeclaration(::std::boxed::Box<AnnotationTypeDeclaration<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
    ModuleDeclaration(::std::boxed::Box<ModuleDeclaration<'tree>>),
    PackageDeclaration(::std::boxed::Box<PackageDeclaration<'tree>>),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declaration<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation_type_declaration" => Ok(Self::AnnotationTypeDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "module_declaration" => Ok(Self::ModuleDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "package_declaration" => Ok(Self::PackageDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PackageDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Declaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotationTypeDeclaration(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::PackageDeclaration(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'tree> {
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    InstanceofExpression(::std::boxed::Box<InstanceofExpression<'tree>>),
    LambdaExpression(::std::boxed::Box<LambdaExpression<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    SwitchExpression(::std::boxed::Box<SwitchExpression<'tree>>),
    TernaryExpression(::std::boxed::Box<TernaryExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    UpdateExpression(::std::boxed::Box<UpdateExpression<'tree>>),
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
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "instanceof_expression" => Ok(Self::InstanceofExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InstanceofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lambda_expression" => Ok(Self::LambdaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_expression" => Ok(Self::SwitchExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::BinaryExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::InstanceofExpression(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::SwitchExpression(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDirective<'tree> {
    ExportsModuleDirective(::std::boxed::Box<ExportsModuleDirective<'tree>>),
    OpensModuleDirective(::std::boxed::Box<OpensModuleDirective<'tree>>),
    ProvidesModuleDirective(::std::boxed::Box<ProvidesModuleDirective<'tree>>),
    RequiresModuleDirective(::std::boxed::Box<RequiresModuleDirective<'tree>>),
    UsesModuleDirective(::std::boxed::Box<UsesModuleDirective<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleDirective<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "exports_module_directive" => Ok(Self::ExportsModuleDirective(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExportsModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "opens_module_directive" => Ok(Self::OpensModuleDirective(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OpensModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "provides_module_directive" => Ok(Self::ProvidesModuleDirective(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ProvidesModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "requires_module_directive" => Ok(Self::RequiresModuleDirective(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RequiresModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "uses_module_directive" => Ok(Self::UsesModuleDirective(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UsesModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExportsModuleDirective(inner) => inner.span(),
            Self::OpensModuleDirective(inner) => inner.span(),
            Self::ProvidesModuleDirective(inner) => inner.span(),
            Self::RequiresModuleDirective(inner) => inner.span(),
            Self::UsesModuleDirective(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryExpression<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    ArrayAccess(::std::boxed::Box<ArrayAccess<'tree>>),
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    ClassLiteral(::std::boxed::Box<ClassLiteral<'tree>>),
    FieldAccess(::std::boxed::Box<FieldAccess<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MethodInvocation(::std::boxed::Box<MethodInvocation<'tree>>),
    MethodReference(::std::boxed::Box<MethodReference<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    TemplateExpression(::std::boxed::Box<TemplateExpression<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimaryExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_access" => Ok(Self::ArrayAccess(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayAccess as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "array_creation_expression" => Ok(Self::ArrayCreationExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "class_literal" => Ok(Self::ClassLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_access" => Ok(Self::FieldAccess(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldAccess as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "method_invocation" => Ok(Self::MethodInvocation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodInvocation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "method_reference" => Ok(Self::MethodReference(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodReference as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_creation_expression" => Ok(Self::ObjectCreationExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "template_expression" => Ok(Self::TemplateExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TemplateExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "this" => Ok(Self::This(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <This as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for PrimaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::ArrayAccess(inner) => inner.span(),
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::ClassLiteral(inner) => inner.span(),
            Self::FieldAccess(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MethodInvocation(inner) => inner.span(),
            Self::MethodReference(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::TemplateExpression(inner) => inner.span(),
            Self::This(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'tree> {
    Semicolon(::treesitter_types::Span),
    AssertStatement(::std::boxed::Box<AssertStatement<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnhancedForStatement(::std::boxed::Box<EnhancedForStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LocalVariableDeclaration(::std::boxed::Box<LocalVariableDeclaration<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SwitchExpression(::std::boxed::Box<SwitchExpression<'tree>>),
    SynchronizedStatement(::std::boxed::Box<SynchronizedStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    TryWithResourcesStatement(::std::boxed::Box<TryWithResourcesStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
    YieldStatement(::std::boxed::Box<YieldStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ";" => Ok(Self::Semicolon(::treesitter_types::Span::from(node))),
            "assert_statement" => Ok(Self::AssertStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssertStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
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
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enhanced_for_statement" => Ok(Self::EnhancedForStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnhancedForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
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
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "local_variable_declaration" => Ok(Self::LocalVariableDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LocalVariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_expression" => Ok(Self::SwitchExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "synchronized_statement" => Ok(Self::SynchronizedStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SynchronizedStatement as ::treesitter_types::FromNode>::from_node(node, src)
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
            "try_with_resources_statement" => Ok(Self::TryWithResourcesStatement(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TryWithResourcesStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "yield_statement" => Ok(Self::YieldStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <YieldStatement as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::Semicolon(span) => *span,
            Self::AssertStatement(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnhancedForStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LocalVariableDeclaration(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SwitchExpression(inner) => inner.span(),
            Self::SynchronizedStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TryWithResourcesStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::YieldStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotatedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AnnotatedTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotatedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <AnnotatedTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: AnnotationArgumentList<'tree>,
    pub name: AnnotationName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Annotation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Annotation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AnnotationArgumentListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotation_argument_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AnnotationArgumentListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnnotationArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationTypeBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AnnotationTypeBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationTypeBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotation_type_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AnnotationTypeBodyChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnnotationTypeBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationTypeDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: AnnotationTypeBody<'tree>,
    pub name: Identifier<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationTypeDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotation_type_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeBody as ::treesitter_types::FromNode>::from_node(child, src)
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnnotationTypeDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationTypeElementDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub dimensions: ::core::option::Option<Dimensions<'tree>>,
    pub name: Identifier<'tree>,
    pub r#type: UnannotatedType<'tree>,
    pub value: ::core::option::Option<AnnotationTypeElementDeclarationValue<'tree>>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationTypeElementDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotation_type_element_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            dimensions: match node.child_by_field_name("dimensions") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
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
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeElementDeclarationValue as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnnotationTypeElementDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayAccess<'tree> {
    pub span: ::treesitter_types::Span,
    pub array: PrimaryExpression<'tree>,
    pub index: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayAccess<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_access");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            array: {
                let child = node
                    .child_by_field_name("array")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("array", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("index", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayAccess<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub dimensions: ::std::vec::Vec<ArrayCreationExpressionDimensions<'tree>>,
    pub r#type: SimpleType<'tree>,
    pub value: ::core::option::Option<ArrayInitializer<'tree>>,
    pub children: ::std::vec::Vec<ArrayCreationExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayCreationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_creation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            dimensions: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("dimensions", &mut cursor) {
                    items
                        .push(
                            ::treesitter_types::runtime::maybe_grow_stack(|| <ArrayCreationExpressionDimensions as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            ))?,
                        );
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SimpleType as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayInitializer as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items
                        .push(
                            ::treesitter_types::runtime::maybe_grow_stack(|| <ArrayCreationExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArrayCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArrayInitializerChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ArrayInitializerChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayType<'tree> {
    pub span: ::treesitter_types::Span,
    pub dimensions: Dimensions<'tree>,
    pub element: UnannotatedType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            dimensions: {
                let child = node.child_by_field_name("dimensions").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("dimensions", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            element: {
                let child = node.child_by_field_name("element").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("element", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct AssertStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssertStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AssignmentExpressionLeft<'tree>,
    pub operator: AssignmentExpressionOperator,
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
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentExpressionOperator as ::treesitter_types::FromNode>::from_node(
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
pub struct Asterisk<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Asterisk<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "asterisk");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Asterisk<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Asterisk<'_> {
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
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Block<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                        <Statement as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct BreakStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Identifier<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::std::vec::Vec<Type<'tree>>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CastExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "cast_expression");
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
impl ::treesitter_types::Spanned for CastExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub children: CatchFormalParameter<'tree>,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <CatchFormalParameter as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <CatchFormalParameter as ::treesitter_types::FromNode>::from_node(
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
                    <CatchFormalParameter as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct CatchFormalParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub dimensions: ::core::option::Option<Dimensions<'tree>>,
    pub name: CatchFormalParameterName<'tree>,
    pub children: ::std::vec::Vec<CatchFormalParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchFormalParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "catch_formal_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            dimensions: match node.child_by_field_name("dimensions") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CatchFormalParameterName as ::treesitter_types::FromNode>::from_node(
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <CatchFormalParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CatchFormalParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UnannotatedType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "catch_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CatchType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassBodyChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassBodyChildren as ::treesitter_types::FromNode>::from_node(child, src)
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
    pub interfaces: ::core::option::Option<SuperInterfaces<'tree>>,
    pub name: Identifier<'tree>,
    pub permits: ::core::option::Option<Permits<'tree>>,
    pub superclass: ::core::option::Option<Superclass<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
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
            interfaces: match node.child_by_field_name("interfaces") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SuperInterfaces as ::treesitter_types::FromNode>::from_node(child, src)
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
            permits: match node.child_by_field_name("permits") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Permits as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            superclass: match node.child_by_field_name("superclass") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Superclass as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct ClassLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: UnannotatedType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <UnannotatedType as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <UnannotatedType as ::treesitter_types::FromNode>::from_node(
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
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactConstructorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub name: Identifier<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompactConstructorDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compact_constructor_declaration");
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompactConstructorDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstantDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<VariableDeclarator<'tree>>,
    pub r#type: UnannotatedType<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstantDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constant_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VariableDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstantDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConstructorBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ConstructorBodyChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstructorBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ConstructorBody<'tree>,
    pub name: Identifier<'tree>,
    pub parameters: FormalParameters<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<ConstructorDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstructorBody as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <ConstructorDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Identifier<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimensions<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DimensionsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Dimensions<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dimensions");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <DimensionsChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Dimensions<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimensionsExpr<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DimensionsExprChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DimensionsExpr<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dimensions_expr");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <DimensionsExprChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DimensionsExpr<'_> {
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
pub struct ElementValueArrayInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ElementValueArrayInitializerChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementValueArrayInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "element_value_array_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            ::treesitter_types::runtime::maybe_grow_stack(|| <ElementValueArrayInitializerChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ElementValueArrayInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementValuePair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: Identifier<'tree>,
    pub value: ElementValuePairValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementValuePair<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "element_value_pair");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("key", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ElementValuePairValue as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElementValuePair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnhancedForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub dimensions: ::core::option::Option<Dimensions<'tree>>,
    pub name: EnhancedForStatementName<'tree>,
    pub r#type: UnannotatedType<'tree>,
    pub value: Expression<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnhancedForStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enhanced_for_statement");
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
            dimensions: match node.child_by_field_name("dimensions") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnhancedForStatementName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnhancedForStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <EnumBodyChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumBodyDeclarations<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumBodyDeclarationsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumBodyDeclarations<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_body_declarations");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EnumBodyDeclarationsChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumBodyDeclarations<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumConstant<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<ArgumentList<'tree>>,
    pub body: ::core::option::Option<ClassBody<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumConstant<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_constant");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassBody as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumConstant<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: EnumBody<'tree>,
    pub interfaces: ::core::option::Option<SuperInterfaces<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            interfaces: match node.child_by_field_name("interfaces") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SuperInterfaces as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplicitConstructorInvocation<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ArgumentList<'tree>,
    pub constructor: ExplicitConstructorInvocationConstructor<'tree>,
    pub object: ::core::option::Option<PrimaryExpression<'tree>>,
    pub type_arguments: ::core::option::Option<TypeArguments<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExplicitConstructorInvocation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "explicit_constructor_invocation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            constructor: {
                let child = node.child_by_field_name("constructor").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constructor", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExplicitConstructorInvocationConstructor as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )
                })?
            },
            object: match node.child_by_field_name("object") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
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
impl ::treesitter_types::Spanned for ExplicitConstructorInvocation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportsModuleDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub modules: ::std::vec::Vec<ExportsModuleDirectiveModules<'tree>>,
    pub package: ExportsModuleDirectivePackage<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportsModuleDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exports_module_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            modules: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("modules", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ExportsModuleDirectiveModules as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
            package: {
                let child = node.child_by_field_name("package").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("package", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExportsModuleDirectivePackage as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExportsModuleDirective<'_> {
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
pub struct ExtendsInterfaces<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtendsInterfaces<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extends_interfaces");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <TypeList as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <TypeList as ::treesitter_types::FromNode>::from_node(
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
                    <TypeList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExtendsInterfaces<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAccess<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: FieldAccessField<'tree>,
    pub object: FieldAccessObject<'tree>,
    pub children: ::core::option::Option<Super<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldAccess<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_access");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldAccessField as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldAccessObject as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Super as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldAccess<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<VariableDeclarator<'tree>>,
    pub r#type: UnannotatedType<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VariableDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct FinallyClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for FinallyClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatingPointType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FloatingPointType<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "floating_point_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FloatingPointType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FloatingPointType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ::core::option::Option<Expression<'tree>>,
    pub init: ::std::vec::Vec<ForStatementInit<'tree>>,
    pub update: ::std::vec::Vec<Expression<'tree>>,
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
            condition: match node.child_by_field_name("condition") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            init: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("init", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ForStatementInit as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            update: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("update", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormalParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub dimensions: ::core::option::Option<Dimensions<'tree>>,
    pub name: FormalParameterName<'tree>,
    pub r#type: UnannotatedType<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormalParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "formal_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            dimensions: match node.child_by_field_name("dimensions") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameterName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for FormalParameter<'_> {
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
pub struct GenericType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GenericTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <GenericTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guard<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Guard<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "guard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for Guard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<Statement<'tree>>,
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
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct ImportDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ImportDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ImportDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferredParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InferredParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inferred_parameters");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for InferredParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub pattern: ::core::option::Option<RecordPattern<'tree>>,
    pub right: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instanceof_expression");
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
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordPattern as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            right: match node.child_by_field_name("right") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for InstanceofExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegralType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntegralType<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "integral_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IntegralType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IntegralType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InterfaceBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interface_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <InterfaceBodyChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InterfaceBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: InterfaceBody<'tree>,
    pub name: Identifier<'tree>,
    pub permits: ::core::option::Option<Permits<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<InterfaceDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interface_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InterfaceBody as ::treesitter_types::FromNode>::from_node(child, src)
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
            permits: match node.child_by_field_name("permits") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Permits as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <InterfaceDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LabeledStatementChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <LabeledStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: LambdaExpressionBody<'tree>,
    pub parameters: LambdaExpressionParameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LambdaExpressionBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LambdaExpressionParameters as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LambdaExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalVariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<VariableDeclarator<'tree>>,
    pub r#type: UnannotatedType<'tree>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalVariableDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_variable_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VariableDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for LocalVariableDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerAnnotation<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: MarkerAnnotationName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MarkerAnnotation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "marker_annotation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotationName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MarkerAnnotation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub dimensions: ::core::option::Option<Dimensions<'tree>>,
    pub name: Identifier<'tree>,
    pub parameters: FormalParameters<'tree>,
    pub r#type: UnannotatedType<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<MethodDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            dimensions: match node.child_by_field_name("dimensions") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
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
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <MethodDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodInvocation<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ArgumentList<'tree>,
    pub name: Identifier<'tree>,
    pub object: ::core::option::Option<MethodInvocationObject<'tree>>,
    pub type_arguments: ::core::option::Option<TypeArguments<'tree>>,
    pub children: ::core::option::Option<Super<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodInvocation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_invocation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
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
            object: match node.child_by_field_name("object") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodInvocationObject as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            type_arguments: match node.child_by_field_name("type_arguments") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Super as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodInvocation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodReference<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MethodReferenceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodReference<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_reference");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <MethodReferenceChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodReference<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modifiers<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModifiersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Modifiers<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ModifiersChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModuleDirective<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ModuleDirective as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ModuleBody<'tree>,
    pub name: ModuleDeclarationName<'tree>,
    pub children: ::std::vec::Vec<ModuleDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ModuleBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ModuleDeclarationName as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <ModuleDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultilineStringFragment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MultilineStringFragment<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "multiline_string_fragment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MultilineStringFragment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MultilineStringFragment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ArgumentList<'tree>,
    pub r#type: SimpleType<'tree>,
    pub type_arguments: ::core::option::Option<TypeArguments<'tree>>,
    pub children: ::std::vec::Vec<ObjectCreationExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectCreationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_creation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SimpleType as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_arguments: match node.child_by_field_name("type_arguments") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items
                        .push(
                            ::treesitter_types::runtime::maybe_grow_stack(|| <ObjectCreationExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ObjectCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpensModuleDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub modules: ::std::vec::Vec<OpensModuleDirectiveModules<'tree>>,
    pub package: OpensModuleDirectivePackage<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OpensModuleDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "opens_module_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            modules: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("modules", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <OpensModuleDirectiveModules as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
            package: {
                let child = node.child_by_field_name("package").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("package", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OpensModuleDirectivePackage as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for OpensModuleDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PackageDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <PackageDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PackageDeclaration<'_> {
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
pub struct Pattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <PatternChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <PatternChildren as ::treesitter_types::FromNode>::from_node(
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
                    <PatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permits<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Permits<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "permits");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <TypeList as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <TypeList as ::treesitter_types::FromNode>::from_node(
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
                    <TypeList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Permits<'_> {
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
pub struct ProvidesModuleDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub provided: ProvidesModuleDirectiveProvided<'tree>,
    pub provider: ::std::vec::Vec<ProvidesModuleDirectiveProvider<'tree>>,
    pub children: ProvidesModuleDirectiveChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProvidesModuleDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "provides_module_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            provided: {
                let child = node.child_by_field_name("provided").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("provided", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ProvidesModuleDirectiveProvided as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            provider: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("provider", &mut cursor) {
                    items
                        .push(
                            ::treesitter_types::runtime::maybe_grow_stack(|| <ProvidesModuleDirectiveProvider as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            ))?,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ProvidesModuleDirectiveChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ProvidesModuleDirectiveChildren as ::treesitter_types::FromNode>::from_node(
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
                    <ProvidesModuleDirectiveChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ProvidesModuleDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiverParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ReceiverParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReceiverParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "receiver_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ReceiverParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReceiverParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ClassBody<'tree>,
    pub interfaces: ::core::option::Option<SuperInterfaces<'tree>>,
    pub name: Identifier<'tree>,
    pub parameters: FormalParameters<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::core::option::Option<Modifiers<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record_declaration");
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
            interfaces: match node.child_by_field_name("interfaces") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SuperInterfaces as ::treesitter_types::FromNode>::from_node(child, src)
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
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Modifiers as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecordDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RecordPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <RecordPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordPatternBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RecordPatternBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordPatternBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record_pattern_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <RecordPatternBodyChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecordPatternBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordPatternComponent<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RecordPatternComponentChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordPatternComponent<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record_pattern_component");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <RecordPatternComponentChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecordPatternComponent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiresModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequiresModifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "requires_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RequiresModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RequiresModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiresModuleDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub modifiers: ::std::vec::Vec<RequiresModifier<'tree>>,
    pub module: RequiresModuleDirectiveModule<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequiresModuleDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "requires_module_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            modifiers: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("modifiers", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <RequiresModifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            module: {
                let child = node
                    .child_by_field_name("module")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("module", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RequiresModuleDirectiveModule as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RequiresModuleDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource<'tree> {
    pub span: ::treesitter_types::Span,
    pub dimensions: ::core::option::Option<Dimensions<'tree>>,
    pub name: ::core::option::Option<ResourceName<'tree>>,
    pub r#type: ::core::option::Option<UnannotatedType<'tree>>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::core::option::Option<ResourceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Resource<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "resource");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            dimensions: match node.child_by_field_name("dimensions") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ResourceName as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <ResourceChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Resource<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Resource<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ResourceSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "resource_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Resource as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ResourceSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
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
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct ScopedIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub scope: ScopedIdentifierScope<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            scope: {
                let child = node
                    .child_by_field_name("scope")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("scope", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifierScope as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
    pub children: ::std::vec::Vec<ScopedTypeIdentifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedTypeIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scoped_type_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ScopedTypeIdentifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
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
pub struct SpreadParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SpreadParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpreadParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "spread_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SpreadParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SpreadParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "static_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for StaticInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringInterpolation<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringInterpolation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_interpolation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for StringInterpolation<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct SuperInterfaces<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SuperInterfaces<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "super_interfaces");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <TypeList as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <TypeList as ::treesitter_types::FromNode>::from_node(
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
                    <TypeList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SuperInterfaces<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Superclass<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Superclass<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "superclass");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for Superclass<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <SwitchBlockChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchBlockStatementGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchBlockStatementGroupChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBlockStatementGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_block_statement_group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            ::treesitter_types::runtime::maybe_grow_stack(|| <SwitchBlockStatementGroupChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SwitchBlockStatementGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SwitchBlock<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchBlock as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for SwitchExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchLabel<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchLabelChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchLabel<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_label");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SwitchLabelChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchLabel<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchRule<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchRuleChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchRule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_rule");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SwitchRuleChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchRule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynchronizedStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub children: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SynchronizedStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "synchronized_statement");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
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
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SynchronizedStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub template_argument: StringLiteral<'tree>,
    pub template_processor: PrimaryExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            template_argument: {
                let child = node
                    .child_by_field_name("template_argument")
                    .ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("template_argument", node)
                    })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            template_processor: {
                let child = node
                    .child_by_field_name("template_processor")
                    .ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("template_processor", node)
                    })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateExpression<'_> {
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
    pub children: Expression<'tree>,
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
impl ::treesitter_types::Spanned for ThrowStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Throws<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Throws<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "throws");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for Throws<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub children: ::std::vec::Vec<TryStatementChildren<'tree>>,
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TryStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryWithResourcesStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub resources: ResourceSpecification<'tree>,
    pub children: ::std::vec::Vec<TryWithResourcesStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryWithResourcesStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "try_with_resources_statement");
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
            resources: {
                let child = node.child_by_field_name("resources").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("resources", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ResourceSpecification as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items
                        .push(
                            ::treesitter_types::runtime::maybe_grow_stack(|| <TryWithResourcesStatementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TryWithResourcesStatement<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct TypeBound<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeBound<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_bound");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for TypeBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Type as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeParameter<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                        <TypeParameter as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct TypePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TypePatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: Expression<'tree>,
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
            operand: {
                let child = node.child_by_field_name("operand").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operand", node)
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
    pub children: Expression<'tree>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for UpdateExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsesModuleDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: UsesModuleDirectiveType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsesModuleDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "uses_module_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UsesModuleDirectiveType as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UsesModuleDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub dimensions: ::core::option::Option<Dimensions<'tree>>,
    pub name: VariableDeclaratorName<'tree>,
    pub value: ::core::option::Option<VariableDeclaratorValue<'tree>>,
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
            dimensions: match node.child_by_field_name("dimensions") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
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
                    <VariableDeclaratorValue as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct Wildcard<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<WildcardChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Wildcard<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "wildcard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <WildcardChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Wildcard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for YieldStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
impl ::treesitter_types::Spanned for YieldStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryIntegerLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryIntegerLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binary_integer_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BinaryIntegerLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BinaryIntegerLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockComment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockComment<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BooleanType<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boolean_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BooleanType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BooleanType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecimalFloatingPointLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecimalFloatingPointLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "decimal_floating_point_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DecimalFloatingPointLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DecimalFloatingPointLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecimalIntegerLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecimalIntegerLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "decimal_integer_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DecimalIntegerLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DecimalIntegerLiteral<'_> {
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
pub struct HexFloatingPointLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HexFloatingPointLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hex_floating_point_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HexFloatingPointLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HexFloatingPointLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexIntegerLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HexIntegerLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hex_integer_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HexIntegerLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HexIntegerLiteral<'_> {
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
pub struct LineComment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LineComment<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "line_comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LineComment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LineComment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OctalIntegerLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OctalIntegerLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "octal_integer_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OctalIntegerLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OctalIntegerLiteral<'_> {
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
pub struct TypeIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeIdentifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct UnderscorePattern<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnderscorePattern<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "underscore_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UnderscorePattern<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UnderscorePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoidType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VoidType<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "void_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VoidType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VoidType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotatedTypeChildren<'tree> {
    UnannotatedType(::std::boxed::Box<UnannotatedType<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotatedTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::UnannotatedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for AnnotatedTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UnannotatedType(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationName<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AnnotationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationArgumentListChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    ElementValueArrayInitializer(::std::boxed::Box<ElementValueArrayInitializer<'tree>>),
    ElementValuePair(::std::boxed::Box<ElementValuePair<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "element_value_array_initializer" => Ok(Self::ElementValueArrayInitializer(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ElementValueArrayInitializer as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "element_value_pair" => Ok(Self::ElementValuePair(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ElementValuePair as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for AnnotationArgumentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::ElementValueArrayInitializer(inner) => inner.span(),
            Self::ElementValuePair(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationTypeBodyChildren<'tree> {
    AnnotationTypeDeclaration(::std::boxed::Box<AnnotationTypeDeclaration<'tree>>),
    AnnotationTypeElementDeclaration(::std::boxed::Box<AnnotationTypeElementDeclaration<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationTypeBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation_type_declaration" => Ok(Self::AnnotationTypeDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "annotation_type_element_declaration" => Ok(Self::AnnotationTypeElementDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeElementDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AnnotationTypeBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotationTypeDeclaration(inner) => inner.span(),
            Self::AnnotationTypeElementDeclaration(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnnotationTypeElementDeclarationValue<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    ElementValueArrayInitializer(::std::boxed::Box<ElementValueArrayInitializer<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationTypeElementDeclarationValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "element_value_array_initializer" => Ok(Self::ElementValueArrayInitializer(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ElementValueArrayInitializer as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for AnnotationTypeElementDeclarationValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::ElementValueArrayInitializer(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayCreationExpressionDimensions<'tree> {
    Dimensions(::std::boxed::Box<Dimensions<'tree>>),
    DimensionsExpr(::std::boxed::Box<DimensionsExpr<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayCreationExpressionDimensions<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dimensions" => Ok(Self::Dimensions(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Dimensions as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "dimensions_expr" => Ok(Self::DimensionsExpr(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DimensionsExpr as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArrayCreationExpressionDimensions<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Dimensions(inner) => inner.span(),
            Self::DimensionsExpr(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayCreationExpressionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayCreationExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArrayCreationExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayInitializerChildren<'tree> {
    ArrayInitializer(::std::boxed::Box<ArrayInitializer<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayInitializerChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_initializer" => Ok(Self::ArrayInitializer(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayInitializer as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ArrayInitializerChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayInitializer(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentExpressionLeft<'tree> {
    ArrayAccess(::std::boxed::Box<ArrayAccess<'tree>>),
    FieldAccess(::std::boxed::Box<FieldAccess<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_access" => Ok(Self::ArrayAccess(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayAccess as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_access" => Ok(Self::FieldAccess(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldAccess as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for AssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayAccess(inner) => inner.span(),
            Self::FieldAccess(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
    CaretEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
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
    GtGtGt(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
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
            Self::Caret(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatchFormalParameterName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    UnderscorePattern(::std::boxed::Box<UnderscorePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchFormalParameterName<'tree> {
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
            "underscore_pattern" => Ok(Self::UnderscorePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnderscorePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CatchFormalParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::UnderscorePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatchFormalParameterChildren<'tree> {
    CatchType(::std::boxed::Box<CatchType<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchFormalParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "catch_type" => Ok(Self::CatchType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CatchType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CatchFormalParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CatchType(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassBodyChildren<'tree> {
    AnnotationTypeDeclaration(::std::boxed::Box<AnnotationTypeDeclaration<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    CompactConstructorDeclaration(::std::boxed::Box<CompactConstructorDeclaration<'tree>>),
    ConstructorDeclaration(::std::boxed::Box<ConstructorDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
    StaticInitializer(::std::boxed::Box<StaticInitializer<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation_type_declaration" => Ok(Self::AnnotationTypeDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compact_constructor_declaration" => Ok(Self::CompactConstructorDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompactConstructorDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "constructor_declaration" => Ok(Self::ConstructorDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "method_declaration" => Ok(Self::MethodDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "static_initializer" => Ok(Self::StaticInitializer(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StaticInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotationTypeDeclaration(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::CompactConstructorDeclaration(inner) => inner.span(),
            Self::ConstructorDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
            Self::StaticInitializer(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructorBodyChildren<'tree> {
    ExplicitConstructorInvocation(::std::boxed::Box<ExplicitConstructorInvocation<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "explicit_constructor_invocation" => Ok(Self::ExplicitConstructorInvocation(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExplicitConstructorInvocation as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
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
impl ::treesitter_types::Spanned for ConstructorBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExplicitConstructorInvocation(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstructorDeclarationChildren<'tree> {
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
    Throws(::std::boxed::Box<Throws<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "throws" => Ok(Self::Throws(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Throws as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Modifiers(inner) => inner.span(),
            Self::Throws(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimensionsChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DimensionsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DimensionsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimensionsExprChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DimensionsExprChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for DimensionsExprChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementValueArrayInitializerChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    ElementValueArrayInitializer(::std::boxed::Box<ElementValueArrayInitializer<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementValueArrayInitializerChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "element_value_array_initializer" => Ok(Self::ElementValueArrayInitializer(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ElementValueArrayInitializer as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ElementValueArrayInitializerChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::ElementValueArrayInitializer(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementValuePairValue<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    ElementValueArrayInitializer(::std::boxed::Box<ElementValueArrayInitializer<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementValuePairValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "element_value_array_initializer" => Ok(Self::ElementValueArrayInitializer(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ElementValueArrayInitializer as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ElementValuePairValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::ElementValueArrayInitializer(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnhancedForStatementName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    UnderscorePattern(::std::boxed::Box<UnderscorePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnhancedForStatementName<'tree> {
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
            "underscore_pattern" => Ok(Self::UnderscorePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnderscorePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnhancedForStatementName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::UnderscorePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumBodyChildren<'tree> {
    EnumBodyDeclarations(::std::boxed::Box<EnumBodyDeclarations<'tree>>),
    EnumConstant(::std::boxed::Box<EnumConstant<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "enum_body_declarations" => Ok(Self::EnumBodyDeclarations(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumBodyDeclarations as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_constant" => Ok(Self::EnumConstant(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumConstant as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EnumBodyDeclarations(inner) => inner.span(),
            Self::EnumConstant(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumBodyDeclarationsChildren<'tree> {
    AnnotationTypeDeclaration(::std::boxed::Box<AnnotationTypeDeclaration<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    CompactConstructorDeclaration(::std::boxed::Box<CompactConstructorDeclaration<'tree>>),
    ConstructorDeclaration(::std::boxed::Box<ConstructorDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
    StaticInitializer(::std::boxed::Box<StaticInitializer<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumBodyDeclarationsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation_type_declaration" => Ok(Self::AnnotationTypeDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compact_constructor_declaration" => Ok(Self::CompactConstructorDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompactConstructorDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "constructor_declaration" => Ok(Self::ConstructorDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "method_declaration" => Ok(Self::MethodDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "static_initializer" => Ok(Self::StaticInitializer(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StaticInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumBodyDeclarationsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotationTypeDeclaration(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::CompactConstructorDeclaration(inner) => inner.span(),
            Self::ConstructorDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
            Self::StaticInitializer(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplicitConstructorInvocationConstructor<'tree> {
    Super(::std::boxed::Box<Super<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
    for ExplicitConstructorInvocationConstructor<'tree>
{
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "this" => Ok(Self::This(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <This as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExplicitConstructorInvocationConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Super(inner) => inner.span(),
            Self::This(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportsModuleDirectiveModules<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportsModuleDirectiveModules<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExportsModuleDirectiveModules<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportsModuleDirectivePackage<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportsModuleDirectivePackage<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExportsModuleDirectivePackage<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldAccessField<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldAccessField<'tree> {
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
            "this" => Ok(Self::This(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <This as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldAccessField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::This(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldAccessObject<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldAccessObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for FieldAccessObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementInit<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LocalVariableDeclaration(::std::boxed::Box<LocalVariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementInit<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "local_variable_declaration" => Ok(Self::LocalVariableDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LocalVariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ForStatementInit<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LocalVariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormalParameterName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    UnderscorePattern(::std::boxed::Box<UnderscorePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormalParameterName<'tree> {
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
            "underscore_pattern" => Ok(Self::UnderscorePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnderscorePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FormalParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::UnderscorePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormalParametersChildren<'tree> {
    FormalParameter(::std::boxed::Box<FormalParameter<'tree>>),
    ReceiverParameter(::std::boxed::Box<ReceiverParameter<'tree>>),
    SpreadParameter(::std::boxed::Box<SpreadParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormalParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "formal_parameter" => Ok(Self::FormalParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "receiver_parameter" => Ok(Self::ReceiverParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReceiverParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "spread_parameter" => Ok(Self::SpreadParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SpreadParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FormalParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FormalParameter(inner) => inner.span(),
            Self::ReceiverParameter(inner) => inner.span(),
            Self::SpreadParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericTypeChildren<'tree> {
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeArguments(::std::boxed::Box<TypeArguments<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_arguments" => Ok(Self::TypeArguments(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for GenericTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeArguments(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportDeclarationChildren<'tree> {
    Asterisk(::std::boxed::Box<Asterisk<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "asterisk" => Ok(Self::Asterisk(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Asterisk as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ImportDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Asterisk(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceBodyChildren<'tree> {
    AnnotationTypeDeclaration(::std::boxed::Box<AnnotationTypeDeclaration<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation_type_declaration" => Ok(Self::AnnotationTypeDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnnotationTypeDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "method_declaration" => Ok(Self::MethodDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InterfaceBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotationTypeDeclaration(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceDeclarationChildren<'tree> {
    ExtendsInterfaces(::std::boxed::Box<ExtendsInterfaces<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "extends_interfaces" => Ok(Self::ExtendsInterfaces(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExtendsInterfaces as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InterfaceDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExtendsInterfaces(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabeledStatementChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledStatementChildren<'tree> {
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
impl ::treesitter_types::Spanned for LabeledStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LambdaExpressionBody<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpressionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for LambdaExpressionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LambdaExpressionParameters<'tree> {
    FormalParameters(::std::boxed::Box<FormalParameters<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InferredParameters(::std::boxed::Box<InferredParameters<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpressionParameters<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "formal_parameters" => Ok(Self::FormalParameters(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "inferred_parameters" => Ok(Self::InferredParameters(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InferredParameters as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaExpressionParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FormalParameters(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InferredParameters(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkerAnnotationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MarkerAnnotationName<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MarkerAnnotationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDeclarationChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
    Throws(::std::boxed::Box<Throws<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "throws" => Ok(Self::Throws(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Throws as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MethodDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
            Self::Throws(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodInvocationObject<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodInvocationObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for MethodInvocationObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodReferenceChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    TypeArguments(::std::boxed::Box<TypeArguments<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodReferenceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_arguments" => Ok(Self::TypeArguments(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Type(::std::boxed::Box::new(v)))
                } else {
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
}
impl ::treesitter_types::Spanned for MethodReferenceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::TypeArguments(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifiersChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModifiersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModifiersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDeclarationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleDeclarationName<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDeclarationChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectCreationExpressionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    ClassBody(::std::boxed::Box<ClassBody<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectCreationExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class_body" => Ok(Self::ClassBody(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassBody as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ObjectCreationExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::ClassBody(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpensModuleDirectiveModules<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OpensModuleDirectiveModules<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OpensModuleDirectiveModules<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpensModuleDirectivePackage<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OpensModuleDirectivePackage<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OpensModuleDirectivePackage<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageDeclarationChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for PackageDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternChildren<'tree> {
    RecordPattern(::std::boxed::Box<RecordPattern<'tree>>),
    TypePattern(::std::boxed::Box<TypePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "record_pattern" => Ok(Self::RecordPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_pattern" => Ok(Self::TypePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::RecordPattern(inner) => inner.span(),
            Self::TypePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramChildren<'tree> {
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProgramChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "method_declaration" => Ok(Self::MethodDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::MethodDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvidesModuleDirectiveProvided<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProvidesModuleDirectiveProvided<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProvidesModuleDirectiveProvided<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvidesModuleDirectiveProvider<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProvidesModuleDirectiveProvider<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProvidesModuleDirectiveProvider<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvidesModuleDirectiveChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProvidesModuleDirectiveChildren<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProvidesModuleDirectiveChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiverParameterChildren<'tree> {
    UnannotatedType(::std::boxed::Box<UnannotatedType<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReceiverParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "this" => Ok(Self::This(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <This as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::UnannotatedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ReceiverParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UnannotatedType(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::This(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordPatternChildren<'tree> {
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    RecordPatternBody(::std::boxed::Box<RecordPatternBody<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
            "record_pattern_body" => Ok(Self::RecordPatternBody(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordPatternBody as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RecordPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::RecordPatternBody(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordPatternBodyChildren<'tree> {
    RecordPattern(::std::boxed::Box<RecordPattern<'tree>>),
    RecordPatternComponent(::std::boxed::Box<RecordPatternComponent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordPatternBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "record_pattern" => Ok(Self::RecordPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "record_pattern_component" => Ok(Self::RecordPatternComponent(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RecordPatternComponent as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RecordPatternBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::RecordPattern(inner) => inner.span(),
            Self::RecordPatternComponent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordPatternComponentChildren<'tree> {
    UnannotatedType(::std::boxed::Box<UnannotatedType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    UnderscorePattern(::std::boxed::Box<UnderscorePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordPatternComponentChildren<'tree> {
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
            "underscore_pattern" => Ok(Self::UnderscorePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnderscorePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::UnannotatedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for RecordPatternComponentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UnannotatedType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::UnderscorePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequiresModuleDirectiveModule<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequiresModuleDirectiveModule<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RequiresModuleDirectiveModule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    UnderscorePattern(::std::boxed::Box<UnderscorePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ResourceName<'tree> {
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
            "underscore_pattern" => Ok(Self::UnderscorePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnderscorePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ResourceName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::UnderscorePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceChildren<'tree> {
    FieldAccess(::std::boxed::Box<FieldAccess<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ResourceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_access" => Ok(Self::FieldAccess(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldAccess as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ResourceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldAccess(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopedIdentifierScope<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedIdentifierScope<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedIdentifierScope<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopedTypeIdentifierChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedTypeIdentifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ScopedTypeIdentifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpreadParameterChildren<'tree> {
    UnannotatedType(::std::boxed::Box<UnannotatedType<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
    VariableDeclarator(::std::boxed::Box<VariableDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpreadParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "variable_declarator" => Ok(Self::VariableDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VariableDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::UnannotatedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SpreadParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UnannotatedType(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
            Self::VariableDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringLiteralChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    MultilineStringFragment(::std::boxed::Box<MultilineStringFragment<'tree>>),
    StringFragment(::std::boxed::Box<StringFragment<'tree>>),
    StringInterpolation(::std::boxed::Box<StringInterpolation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringLiteralChildren<'tree> {
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
            "multiline_string_fragment" => Ok(Self::MultilineStringFragment(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MultilineStringFragment as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "string_fragment" => Ok(Self::StringFragment(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringFragment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_interpolation" => Ok(Self::StringInterpolation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringInterpolation as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::MultilineStringFragment(inner) => inner.span(),
            Self::StringFragment(inner) => inner.span(),
            Self::StringInterpolation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchBlockChildren<'tree> {
    SwitchBlockStatementGroup(::std::boxed::Box<SwitchBlockStatementGroup<'tree>>),
    SwitchRule(::std::boxed::Box<SwitchRule<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "switch_block_statement_group" => Ok(Self::SwitchBlockStatementGroup(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchBlockStatementGroup as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "switch_rule" => Ok(Self::SwitchRule(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchRule as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SwitchBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SwitchBlockStatementGroup(inner) => inner.span(),
            Self::SwitchRule(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchBlockStatementGroupChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    SwitchLabel(::std::boxed::Box<SwitchLabel<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBlockStatementGroupChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "switch_label" => Ok(Self::SwitchLabel(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchLabel as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for SwitchBlockStatementGroupChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::SwitchLabel(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchLabelChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Guard(::std::boxed::Box<Guard<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchLabelChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "guard" => Ok(Self::Guard(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Guard as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pattern" => Ok(Self::Pattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for SwitchLabelChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchRuleChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    SwitchLabel(::std::boxed::Box<SwitchLabel<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchRuleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_label" => Ok(Self::SwitchLabel(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchLabel as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SwitchRuleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::SwitchLabel(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryStatementChildren<'tree> {
    CatchClause(::std::boxed::Box<CatchClause<'tree>>),
    FinallyClause(::std::boxed::Box<FinallyClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "catch_clause" => Ok(Self::CatchClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "finally_clause" => Ok(Self::FinallyClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TryWithResourcesStatementChildren<'tree> {
    CatchClause(::std::boxed::Box<CatchClause<'tree>>),
    FinallyClause(::std::boxed::Box<FinallyClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryWithResourcesStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "catch_clause" => Ok(Self::CatchClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "finally_clause" => Ok(Self::FinallyClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TryWithResourcesStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CatchClause(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeArgumentsChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for TypeArgumentsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeParameterChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    TypeBound(::std::boxed::Box<TypeBound<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_bound" => Ok(Self::TypeBound(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeBound as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for TypeParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::TypeBound(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypePatternChildren<'tree> {
    UnannotatedType(::std::boxed::Box<UnannotatedType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypePatternChildren<'tree> {
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
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnannotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::UnannotatedType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for TypePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UnannotatedType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpressionOperator {
    Bang(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
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
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsesModuleDirectiveType<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsesModuleDirectiveType<'tree> {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UsesModuleDirectiveType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableDeclaratorName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    UnderscorePattern(::std::boxed::Box<UnderscorePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclaratorName<'tree> {
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
            "underscore_pattern" => Ok(Self::UnderscorePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnderscorePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableDeclaratorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::UnderscorePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableDeclaratorValue<'tree> {
    ArrayInitializer(::std::boxed::Box<ArrayInitializer<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclaratorValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_initializer" => Ok(Self::ArrayInitializer(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayInitializer as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for VariableDeclaratorValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayInitializer(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WildcardChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    MarkerAnnotation(::std::boxed::Box<MarkerAnnotation<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WildcardChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "marker_annotation" => Ok(Self::MarkerAnnotation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for WildcardChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Literal(Literal<'tree>),
    SimpleType(SimpleType<'tree>),
    Type(Type<'tree>),
    UnannotatedType(UnannotatedType<'tree>),
    Declaration(Declaration<'tree>),
    Expression(Expression<'tree>),
    ModuleDirective(ModuleDirective<'tree>),
    PrimaryExpression(PrimaryExpression<'tree>),
    Statement(Statement<'tree>),
    AnnotatedType(AnnotatedType<'tree>),
    Annotation(Annotation<'tree>),
    AnnotationArgumentList(AnnotationArgumentList<'tree>),
    AnnotationTypeBody(AnnotationTypeBody<'tree>),
    AnnotationTypeDeclaration(AnnotationTypeDeclaration<'tree>),
    AnnotationTypeElementDeclaration(AnnotationTypeElementDeclaration<'tree>),
    ArgumentList(ArgumentList<'tree>),
    ArrayAccess(ArrayAccess<'tree>),
    ArrayCreationExpression(ArrayCreationExpression<'tree>),
    ArrayInitializer(ArrayInitializer<'tree>),
    ArrayType(ArrayType<'tree>),
    AssertStatement(AssertStatement<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    Asterisk(Asterisk<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Block(Block<'tree>),
    BreakStatement(BreakStatement<'tree>),
    CastExpression(CastExpression<'tree>),
    CatchClause(CatchClause<'tree>),
    CatchFormalParameter(CatchFormalParameter<'tree>),
    CatchType(CatchType<'tree>),
    ClassBody(ClassBody<'tree>),
    ClassDeclaration(ClassDeclaration<'tree>),
    ClassLiteral(ClassLiteral<'tree>),
    CompactConstructorDeclaration(CompactConstructorDeclaration<'tree>),
    ConstantDeclaration(ConstantDeclaration<'tree>),
    ConstructorBody(ConstructorBody<'tree>),
    ConstructorDeclaration(ConstructorDeclaration<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    Dimensions(Dimensions<'tree>),
    DimensionsExpr(DimensionsExpr<'tree>),
    DoStatement(DoStatement<'tree>),
    ElementValueArrayInitializer(ElementValueArrayInitializer<'tree>),
    ElementValuePair(ElementValuePair<'tree>),
    EnhancedForStatement(EnhancedForStatement<'tree>),
    EnumBody(EnumBody<'tree>),
    EnumBodyDeclarations(EnumBodyDeclarations<'tree>),
    EnumConstant(EnumConstant<'tree>),
    EnumDeclaration(EnumDeclaration<'tree>),
    ExplicitConstructorInvocation(ExplicitConstructorInvocation<'tree>),
    ExportsModuleDirective(ExportsModuleDirective<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    ExtendsInterfaces(ExtendsInterfaces<'tree>),
    FieldAccess(FieldAccess<'tree>),
    FieldDeclaration(FieldDeclaration<'tree>),
    FinallyClause(FinallyClause<'tree>),
    FloatingPointType(FloatingPointType<'tree>),
    ForStatement(ForStatement<'tree>),
    FormalParameter(FormalParameter<'tree>),
    FormalParameters(FormalParameters<'tree>),
    GenericType(GenericType<'tree>),
    Guard(Guard<'tree>),
    IfStatement(IfStatement<'tree>),
    ImportDeclaration(ImportDeclaration<'tree>),
    InferredParameters(InferredParameters<'tree>),
    InstanceofExpression(InstanceofExpression<'tree>),
    IntegralType(IntegralType<'tree>),
    InterfaceBody(InterfaceBody<'tree>),
    InterfaceDeclaration(InterfaceDeclaration<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    LambdaExpression(LambdaExpression<'tree>),
    LocalVariableDeclaration(LocalVariableDeclaration<'tree>),
    MarkerAnnotation(MarkerAnnotation<'tree>),
    MethodDeclaration(MethodDeclaration<'tree>),
    MethodInvocation(MethodInvocation<'tree>),
    MethodReference(MethodReference<'tree>),
    Modifiers(Modifiers<'tree>),
    ModuleBody(ModuleBody<'tree>),
    ModuleDeclaration(ModuleDeclaration<'tree>),
    MultilineStringFragment(MultilineStringFragment<'tree>),
    ObjectCreationExpression(ObjectCreationExpression<'tree>),
    OpensModuleDirective(OpensModuleDirective<'tree>),
    PackageDeclaration(PackageDeclaration<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    Pattern(Pattern<'tree>),
    Permits(Permits<'tree>),
    Program(Program<'tree>),
    ProvidesModuleDirective(ProvidesModuleDirective<'tree>),
    ReceiverParameter(ReceiverParameter<'tree>),
    RecordDeclaration(RecordDeclaration<'tree>),
    RecordPattern(RecordPattern<'tree>),
    RecordPatternBody(RecordPatternBody<'tree>),
    RecordPatternComponent(RecordPatternComponent<'tree>),
    RequiresModifier(RequiresModifier<'tree>),
    RequiresModuleDirective(RequiresModuleDirective<'tree>),
    Resource(Resource<'tree>),
    ResourceSpecification(ResourceSpecification<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    ScopedIdentifier(ScopedIdentifier<'tree>),
    ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
    SpreadParameter(SpreadParameter<'tree>),
    StaticInitializer(StaticInitializer<'tree>),
    StringInterpolation(StringInterpolation<'tree>),
    StringLiteral(StringLiteral<'tree>),
    SuperInterfaces(SuperInterfaces<'tree>),
    Superclass(Superclass<'tree>),
    SwitchBlock(SwitchBlock<'tree>),
    SwitchBlockStatementGroup(SwitchBlockStatementGroup<'tree>),
    SwitchExpression(SwitchExpression<'tree>),
    SwitchLabel(SwitchLabel<'tree>),
    SwitchRule(SwitchRule<'tree>),
    SynchronizedStatement(SynchronizedStatement<'tree>),
    TemplateExpression(TemplateExpression<'tree>),
    TernaryExpression(TernaryExpression<'tree>),
    ThrowStatement(ThrowStatement<'tree>),
    Throws(Throws<'tree>),
    TryStatement(TryStatement<'tree>),
    TryWithResourcesStatement(TryWithResourcesStatement<'tree>),
    TypeArguments(TypeArguments<'tree>),
    TypeBound(TypeBound<'tree>),
    TypeList(TypeList<'tree>),
    TypeParameter(TypeParameter<'tree>),
    TypeParameters(TypeParameters<'tree>),
    TypePattern(TypePattern<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UpdateExpression(UpdateExpression<'tree>),
    UsesModuleDirective(UsesModuleDirective<'tree>),
    VariableDeclarator(VariableDeclarator<'tree>),
    WhileStatement(WhileStatement<'tree>),
    Wildcard(Wildcard<'tree>),
    YieldStatement(YieldStatement<'tree>),
    BinaryIntegerLiteral(BinaryIntegerLiteral<'tree>),
    BlockComment(BlockComment<'tree>),
    BooleanType(BooleanType<'tree>),
    CharacterLiteral(CharacterLiteral<'tree>),
    DecimalFloatingPointLiteral(DecimalFloatingPointLiteral<'tree>),
    DecimalIntegerLiteral(DecimalIntegerLiteral<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    HexFloatingPointLiteral(HexFloatingPointLiteral<'tree>),
    HexIntegerLiteral(HexIntegerLiteral<'tree>),
    Identifier(Identifier<'tree>),
    LineComment(LineComment<'tree>),
    NullLiteral(NullLiteral<'tree>),
    OctalIntegerLiteral(OctalIntegerLiteral<'tree>),
    StringFragment(StringFragment<'tree>),
    Super(Super<'tree>),
    This(This<'tree>),
    True(True<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    UnderscorePattern(UnderscorePattern<'tree>),
    VoidType(VoidType<'tree>),
    Unknown(::treesitter_types::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::treesitter_types::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Literal as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Literal)
            .unwrap_or(Self::Unknown(node)),
            "_simple_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SimpleType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SimpleType)
            .unwrap_or(Self::Unknown(node)),
            "_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Type as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Type)
            .unwrap_or(Self::Unknown(node)),
            "_unannotated_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnannotatedType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnannotatedType)
            .unwrap_or(Self::Unknown(node)),
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
            "module_directive" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ModuleDirective)
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
            "annotated_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AnnotatedType)
            .unwrap_or(Self::Unknown(node)),
            "annotation" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Annotation)
            .unwrap_or(Self::Unknown(node)),
            "annotation_argument_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AnnotationArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AnnotationArgumentList)
            .unwrap_or(Self::Unknown(node)),
            "annotation_type_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AnnotationTypeBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AnnotationTypeBody)
            .unwrap_or(Self::Unknown(node)),
            "annotation_type_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AnnotationTypeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AnnotationTypeDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "annotation_type_element_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AnnotationTypeElementDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
            })
            .map(Self::AnnotationTypeElementDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "argument_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArgumentList)
            .unwrap_or(Self::Unknown(node)),
            "array_access" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayAccess as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayAccess)
            .unwrap_or(Self::Unknown(node)),
            "array_creation_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayCreationExpression)
            .unwrap_or(Self::Unknown(node)),
            "array_initializer" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayInitializer as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayInitializer)
            .unwrap_or(Self::Unknown(node)),
            "array_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayType)
            .unwrap_or(Self::Unknown(node)),
            "assert_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AssertStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AssertStatement)
            .unwrap_or(Self::Unknown(node)),
            "assignment_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AssignmentExpression)
            .unwrap_or(Self::Unknown(node)),
            "asterisk" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Asterisk as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Asterisk)
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
            "break_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BreakStatement)
            .unwrap_or(Self::Unknown(node)),
            "cast_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CastExpression)
            .unwrap_or(Self::Unknown(node)),
            "catch_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CatchClause)
            .unwrap_or(Self::Unknown(node)),
            "catch_formal_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CatchFormalParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CatchFormalParameter)
            .unwrap_or(Self::Unknown(node)),
            "catch_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CatchType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CatchType)
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
            "class_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClassLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassLiteral)
            .unwrap_or(Self::Unknown(node)),
            "compact_constructor_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CompactConstructorDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
            })
            .map(Self::CompactConstructorDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "constant_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConstantDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "constructor_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConstructorBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConstructorBody)
            .unwrap_or(Self::Unknown(node)),
            "constructor_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConstructorDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "continue_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ContinueStatement)
            .unwrap_or(Self::Unknown(node)),
            "dimensions" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Dimensions as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Dimensions)
            .unwrap_or(Self::Unknown(node)),
            "dimensions_expr" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DimensionsExpr as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DimensionsExpr)
            .unwrap_or(Self::Unknown(node)),
            "do_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DoStatement)
            .unwrap_or(Self::Unknown(node)),
            "element_value_array_initializer" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ElementValueArrayInitializer as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ElementValueArrayInitializer)
            .unwrap_or(Self::Unknown(node)),
            "element_value_pair" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ElementValuePair as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ElementValuePair)
            .unwrap_or(Self::Unknown(node)),
            "enhanced_for_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnhancedForStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnhancedForStatement)
            .unwrap_or(Self::Unknown(node)),
            "enum_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumBody)
            .unwrap_or(Self::Unknown(node)),
            "enum_body_declarations" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumBodyDeclarations as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumBodyDeclarations)
            .unwrap_or(Self::Unknown(node)),
            "enum_constant" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumConstant as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumConstant)
            .unwrap_or(Self::Unknown(node)),
            "enum_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "explicit_constructor_invocation" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExplicitConstructorInvocation as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
            })
            .map(Self::ExplicitConstructorInvocation)
            .unwrap_or(Self::Unknown(node)),
            "exports_module_directive" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExportsModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExportsModuleDirective)
            .unwrap_or(Self::Unknown(node)),
            "expression_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExpressionStatement)
            .unwrap_or(Self::Unknown(node)),
            "extends_interfaces" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExtendsInterfaces as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExtendsInterfaces)
            .unwrap_or(Self::Unknown(node)),
            "field_access" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldAccess as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldAccess)
            .unwrap_or(Self::Unknown(node)),
            "field_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "finally_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FinallyClause)
            .unwrap_or(Self::Unknown(node)),
            "floating_point_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FloatingPointType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FloatingPointType)
            .unwrap_or(Self::Unknown(node)),
            "for_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForStatement)
            .unwrap_or(Self::Unknown(node)),
            "formal_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FormalParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FormalParameter)
            .unwrap_or(Self::Unknown(node)),
            "formal_parameters" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FormalParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FormalParameters)
            .unwrap_or(Self::Unknown(node)),
            "generic_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GenericType)
            .unwrap_or(Self::Unknown(node)),
            "guard" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Guard as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Guard)
            .unwrap_or(Self::Unknown(node)),
            "if_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IfStatement)
            .unwrap_or(Self::Unknown(node)),
            "import_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ImportDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "inferred_parameters" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InferredParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InferredParameters)
            .unwrap_or(Self::Unknown(node)),
            "instanceof_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InstanceofExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InstanceofExpression)
            .unwrap_or(Self::Unknown(node)),
            "integral_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IntegralType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IntegralType)
            .unwrap_or(Self::Unknown(node)),
            "interface_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InterfaceBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InterfaceBody)
            .unwrap_or(Self::Unknown(node)),
            "interface_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InterfaceDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "labeled_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LabeledStatement)
            .unwrap_or(Self::Unknown(node)),
            "lambda_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LambdaExpression)
            .unwrap_or(Self::Unknown(node)),
            "local_variable_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LocalVariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LocalVariableDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "marker_annotation" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MarkerAnnotation as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MarkerAnnotation)
            .unwrap_or(Self::Unknown(node)),
            "method_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MethodDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "method_invocation" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MethodInvocation as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MethodInvocation)
            .unwrap_or(Self::Unknown(node)),
            "method_reference" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MethodReference as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MethodReference)
            .unwrap_or(Self::Unknown(node)),
            "modifiers" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Modifiers)
            .unwrap_or(Self::Unknown(node)),
            "module_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ModuleBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ModuleBody)
            .unwrap_or(Self::Unknown(node)),
            "module_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ModuleDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "multiline_string_fragment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MultilineStringFragment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MultilineStringFragment)
            .unwrap_or(Self::Unknown(node)),
            "object_creation_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ObjectCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ObjectCreationExpression)
            .unwrap_or(Self::Unknown(node)),
            "opens_module_directive" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OpensModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OpensModuleDirective)
            .unwrap_or(Self::Unknown(node)),
            "package_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PackageDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PackageDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParenthesizedExpression)
            .unwrap_or(Self::Unknown(node)),
            "pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pattern)
            .unwrap_or(Self::Unknown(node)),
            "permits" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Permits as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Permits)
            .unwrap_or(Self::Unknown(node)),
            "program" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Program as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Program)
            .unwrap_or(Self::Unknown(node)),
            "provides_module_directive" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ProvidesModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ProvidesModuleDirective)
            .unwrap_or(Self::Unknown(node)),
            "receiver_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReceiverParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReceiverParameter)
            .unwrap_or(Self::Unknown(node)),
            "record_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RecordDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "record_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RecordPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RecordPattern)
            .unwrap_or(Self::Unknown(node)),
            "record_pattern_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RecordPatternBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RecordPatternBody)
            .unwrap_or(Self::Unknown(node)),
            "record_pattern_component" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RecordPatternComponent as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RecordPatternComponent)
            .unwrap_or(Self::Unknown(node)),
            "requires_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RequiresModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RequiresModifier)
            .unwrap_or(Self::Unknown(node)),
            "requires_module_directive" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RequiresModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RequiresModuleDirective)
            .unwrap_or(Self::Unknown(node)),
            "resource" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Resource as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Resource)
            .unwrap_or(Self::Unknown(node)),
            "resource_specification" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ResourceSpecification as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ResourceSpecification)
            .unwrap_or(Self::Unknown(node)),
            "return_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReturnStatement)
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
            "spread_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SpreadParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SpreadParameter)
            .unwrap_or(Self::Unknown(node)),
            "static_initializer" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StaticInitializer as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StaticInitializer)
            .unwrap_or(Self::Unknown(node)),
            "string_interpolation" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StringInterpolation as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringInterpolation)
            .unwrap_or(Self::Unknown(node)),
            "string_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringLiteral)
            .unwrap_or(Self::Unknown(node)),
            "super_interfaces" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SuperInterfaces as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SuperInterfaces)
            .unwrap_or(Self::Unknown(node)),
            "superclass" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Superclass as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Superclass)
            .unwrap_or(Self::Unknown(node)),
            "switch_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchBlock)
            .unwrap_or(Self::Unknown(node)),
            "switch_block_statement_group" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchBlockStatementGroup as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchBlockStatementGroup)
            .unwrap_or(Self::Unknown(node)),
            "switch_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchExpression)
            .unwrap_or(Self::Unknown(node)),
            "switch_label" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchLabel as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchLabel)
            .unwrap_or(Self::Unknown(node)),
            "switch_rule" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchRule as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchRule)
            .unwrap_or(Self::Unknown(node)),
            "synchronized_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SynchronizedStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SynchronizedStatement)
            .unwrap_or(Self::Unknown(node)),
            "template_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TemplateExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TemplateExpression)
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
            "throws" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Throws as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Throws)
            .unwrap_or(Self::Unknown(node)),
            "try_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TryStatement)
            .unwrap_or(Self::Unknown(node)),
            "try_with_resources_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TryWithResourcesStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TryWithResourcesStatement)
            .unwrap_or(Self::Unknown(node)),
            "type_arguments" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeArguments as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeArguments)
            .unwrap_or(Self::Unknown(node)),
            "type_bound" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeBound as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeBound)
            .unwrap_or(Self::Unknown(node)),
            "type_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeList)
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
            "type_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypePattern)
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
            "uses_module_directive" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UsesModuleDirective as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UsesModuleDirective)
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
            "wildcard" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Wildcard)
            .unwrap_or(Self::Unknown(node)),
            "yield_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <YieldStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::YieldStatement)
            .unwrap_or(Self::Unknown(node)),
            "binary_integer_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BinaryIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BinaryIntegerLiteral)
            .unwrap_or(Self::Unknown(node)),
            "block_comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BlockComment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BlockComment)
            .unwrap_or(Self::Unknown(node)),
            "boolean_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BooleanType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BooleanType)
            .unwrap_or(Self::Unknown(node)),
            "character_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CharacterLiteral)
            .unwrap_or(Self::Unknown(node)),
            "decimal_floating_point_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DecimalFloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DecimalFloatingPointLiteral)
            .unwrap_or(Self::Unknown(node)),
            "decimal_integer_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DecimalIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DecimalIntegerLiteral)
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
            "hex_floating_point_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <HexFloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HexFloatingPointLiteral)
            .unwrap_or(Self::Unknown(node)),
            "hex_integer_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <HexIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HexIntegerLiteral)
            .unwrap_or(Self::Unknown(node)),
            "identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Identifier)
            .unwrap_or(Self::Unknown(node)),
            "line_comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LineComment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LineComment)
            .unwrap_or(Self::Unknown(node)),
            "null_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NullLiteral)
            .unwrap_or(Self::Unknown(node)),
            "octal_integer_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OctalIntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OctalIntegerLiteral)
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
            "type_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "underscore_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnderscorePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnderscorePattern)
            .unwrap_or(Self::Unknown(node)),
            "void_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <VoidType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::VoidType)
            .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::SimpleType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::UnannotatedType(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ModuleDirective(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::AnnotatedType(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::AnnotationArgumentList(inner) => inner.span(),
            Self::AnnotationTypeBody(inner) => inner.span(),
            Self::AnnotationTypeDeclaration(inner) => inner.span(),
            Self::AnnotationTypeElementDeclaration(inner) => inner.span(),
            Self::ArgumentList(inner) => inner.span(),
            Self::ArrayAccess(inner) => inner.span(),
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::ArrayInitializer(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::AssertStatement(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::Asterisk(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::CatchFormalParameter(inner) => inner.span(),
            Self::CatchType(inner) => inner.span(),
            Self::ClassBody(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ClassLiteral(inner) => inner.span(),
            Self::CompactConstructorDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ConstructorBody(inner) => inner.span(),
            Self::ConstructorDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Dimensions(inner) => inner.span(),
            Self::DimensionsExpr(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ElementValueArrayInitializer(inner) => inner.span(),
            Self::ElementValuePair(inner) => inner.span(),
            Self::EnhancedForStatement(inner) => inner.span(),
            Self::EnumBody(inner) => inner.span(),
            Self::EnumBodyDeclarations(inner) => inner.span(),
            Self::EnumConstant(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExplicitConstructorInvocation(inner) => inner.span(),
            Self::ExportsModuleDirective(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ExtendsInterfaces(inner) => inner.span(),
            Self::FieldAccess(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
            Self::FloatingPointType(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FormalParameter(inner) => inner.span(),
            Self::FormalParameters(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InferredParameters(inner) => inner.span(),
            Self::InstanceofExpression(inner) => inner.span(),
            Self::IntegralType(inner) => inner.span(),
            Self::InterfaceBody(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::LocalVariableDeclaration(inner) => inner.span(),
            Self::MarkerAnnotation(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::MethodInvocation(inner) => inner.span(),
            Self::MethodReference(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
            Self::ModuleBody(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::MultilineStringFragment(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::OpensModuleDirective(inner) => inner.span(),
            Self::PackageDeclaration(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Permits(inner) => inner.span(),
            Self::Program(inner) => inner.span(),
            Self::ProvidesModuleDirective(inner) => inner.span(),
            Self::ReceiverParameter(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
            Self::RecordPattern(inner) => inner.span(),
            Self::RecordPatternBody(inner) => inner.span(),
            Self::RecordPatternComponent(inner) => inner.span(),
            Self::RequiresModifier(inner) => inner.span(),
            Self::RequiresModuleDirective(inner) => inner.span(),
            Self::Resource(inner) => inner.span(),
            Self::ResourceSpecification(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::SpreadParameter(inner) => inner.span(),
            Self::StaticInitializer(inner) => inner.span(),
            Self::StringInterpolation(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::SuperInterfaces(inner) => inner.span(),
            Self::Superclass(inner) => inner.span(),
            Self::SwitchBlock(inner) => inner.span(),
            Self::SwitchBlockStatementGroup(inner) => inner.span(),
            Self::SwitchExpression(inner) => inner.span(),
            Self::SwitchLabel(inner) => inner.span(),
            Self::SwitchRule(inner) => inner.span(),
            Self::SynchronizedStatement(inner) => inner.span(),
            Self::TemplateExpression(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::Throws(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TryWithResourcesStatement(inner) => inner.span(),
            Self::TypeArguments(inner) => inner.span(),
            Self::TypeBound(inner) => inner.span(),
            Self::TypeList(inner) => inner.span(),
            Self::TypeParameter(inner) => inner.span(),
            Self::TypeParameters(inner) => inner.span(),
            Self::TypePattern(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::UsesModuleDirective(inner) => inner.span(),
            Self::VariableDeclarator(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
            Self::YieldStatement(inner) => inner.span(),
            Self::BinaryIntegerLiteral(inner) => inner.span(),
            Self::BlockComment(inner) => inner.span(),
            Self::BooleanType(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::DecimalFloatingPointLiteral(inner) => inner.span(),
            Self::DecimalIntegerLiteral(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::HexFloatingPointLiteral(inner) => inner.span(),
            Self::HexIntegerLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::LineComment(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OctalIntegerLiteral(inner) => inner.span(),
            Self::StringFragment(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::This(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::UnderscorePattern(inner) => inner.span(),
            Self::VoidType(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
