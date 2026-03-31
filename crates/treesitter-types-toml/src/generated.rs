#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArrayChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Array<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                            <ArrayChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Array<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DocumentChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Document<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "document");
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
                            <DocumentChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Document<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DottedKey<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DottedKeyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DottedKey<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dotted_key");
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
                            <DottedKeyChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DottedKey<'_> {
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
pub struct InlineTable<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pair<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InlineTable<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inline_table");
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
                            <Pair as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InlineTable<'_> {
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
                            <PairChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Pair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedKey<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EscapeSequence<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedKey<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_key");
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
                            <EscapeSequence as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for QuotedKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                            <EscapeSequence as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for String<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TableChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Table<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "table");
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
                            <TableChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Table<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableArrayElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TableArrayElementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TableArrayElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "table_array_element");
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
                            <TableArrayElementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TableArrayElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BareKey<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BareKey<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bare_key");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BareKey<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BareKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
pub struct LocalDate<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalDate<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_date");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LocalDate<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LocalDate<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalDateTime<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalDateTime<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_date_time");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LocalDateTime<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LocalDateTime<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalTime<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalTime<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_time");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LocalTime<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LocalTime<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetDateTime<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OffsetDateTime<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "offset_date_time");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OffsetDateTime<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OffsetDateTime<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayChildren<'tree> {
    Array(::std::boxed::Box<Array<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    InlineTable(::std::boxed::Box<InlineTable<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    LocalDate(::std::boxed::Box<LocalDate<'tree>>),
    LocalDateTime(::std::boxed::Box<LocalDateTime<'tree>>),
    LocalTime(::std::boxed::Box<LocalTime<'tree>>),
    OffsetDateTime(::std::boxed::Box<OffsetDateTime<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array" => {
                Ok(
                    Self::Array(
                        ::std::boxed::Box::new(
                            <Array as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
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
            "inline_table" => {
                Ok(
                    Self::InlineTable(
                        ::std::boxed::Box::new(
                            <InlineTable as ::treesitter_types::FromNode>::from_node(
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
            "local_date" => {
                Ok(
                    Self::LocalDate(
                        ::std::boxed::Box::new(
                            <LocalDate as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "local_date_time" => {
                Ok(
                    Self::LocalDateTime(
                        ::std::boxed::Box::new(
                            <LocalDateTime as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "local_time" => {
                Ok(
                    Self::LocalTime(
                        ::std::boxed::Box::new(
                            <LocalTime as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "offset_date_time" => {
                Ok(
                    Self::OffsetDateTime(
                        ::std::boxed::Box::new(
                            <OffsetDateTime as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArrayChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Array(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::InlineTable(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::LocalDate(inner) => inner.span(),
            Self::LocalDateTime(inner) => inner.span(),
            Self::LocalTime(inner) => inner.span(),
            Self::OffsetDateTime(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentChildren<'tree> {
    Pair(::std::boxed::Box<Pair<'tree>>),
    Table(::std::boxed::Box<Table<'tree>>),
    TableArrayElement(::std::boxed::Box<TableArrayElement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DocumentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "pair" => {
                Ok(
                    Self::Pair(
                        ::std::boxed::Box::new(
                            <Pair as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "table" => {
                Ok(
                    Self::Table(
                        ::std::boxed::Box::new(
                            <Table as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "table_array_element" => {
                Ok(
                    Self::TableArrayElement(
                        ::std::boxed::Box::new(
                            <TableArrayElement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DocumentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pair(inner) => inner.span(),
            Self::Table(inner) => inner.span(),
            Self::TableArrayElement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DottedKeyChildren<'tree> {
    BareKey(::std::boxed::Box<BareKey<'tree>>),
    DottedKey(::std::boxed::Box<DottedKey<'tree>>),
    QuotedKey(::std::boxed::Box<QuotedKey<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DottedKeyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bare_key" => {
                Ok(
                    Self::BareKey(
                        ::std::boxed::Box::new(
                            <BareKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "dotted_key" => {
                Ok(
                    Self::DottedKey(
                        ::std::boxed::Box::new(
                            <DottedKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quoted_key" => {
                Ok(
                    Self::QuotedKey(
                        ::std::boxed::Box::new(
                            <QuotedKey as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DottedKeyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BareKey(inner) => inner.span(),
            Self::DottedKey(inner) => inner.span(),
            Self::QuotedKey(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairChildren<'tree> {
    Array(::std::boxed::Box<Array<'tree>>),
    BareKey(::std::boxed::Box<BareKey<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    DottedKey(::std::boxed::Box<DottedKey<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    InlineTable(::std::boxed::Box<InlineTable<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    LocalDate(::std::boxed::Box<LocalDate<'tree>>),
    LocalDateTime(::std::boxed::Box<LocalDateTime<'tree>>),
    LocalTime(::std::boxed::Box<LocalTime<'tree>>),
    OffsetDateTime(::std::boxed::Box<OffsetDateTime<'tree>>),
    QuotedKey(::std::boxed::Box<QuotedKey<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array" => {
                Ok(
                    Self::Array(
                        ::std::boxed::Box::new(
                            <Array as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "bare_key" => {
                Ok(
                    Self::BareKey(
                        ::std::boxed::Box::new(
                            <BareKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
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
            "dotted_key" => {
                Ok(
                    Self::DottedKey(
                        ::std::boxed::Box::new(
                            <DottedKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
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
            "inline_table" => {
                Ok(
                    Self::InlineTable(
                        ::std::boxed::Box::new(
                            <InlineTable as ::treesitter_types::FromNode>::from_node(
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
            "local_date" => {
                Ok(
                    Self::LocalDate(
                        ::std::boxed::Box::new(
                            <LocalDate as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "local_date_time" => {
                Ok(
                    Self::LocalDateTime(
                        ::std::boxed::Box::new(
                            <LocalDateTime as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "local_time" => {
                Ok(
                    Self::LocalTime(
                        ::std::boxed::Box::new(
                            <LocalTime as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "offset_date_time" => {
                Ok(
                    Self::OffsetDateTime(
                        ::std::boxed::Box::new(
                            <OffsetDateTime as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "quoted_key" => {
                Ok(
                    Self::QuotedKey(
                        ::std::boxed::Box::new(
                            <QuotedKey as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PairChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Array(inner) => inner.span(),
            Self::BareKey(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::DottedKey(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::InlineTable(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::LocalDate(inner) => inner.span(),
            Self::LocalDateTime(inner) => inner.span(),
            Self::LocalTime(inner) => inner.span(),
            Self::OffsetDateTime(inner) => inner.span(),
            Self::QuotedKey(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableChildren<'tree> {
    BareKey(::std::boxed::Box<BareKey<'tree>>),
    DottedKey(::std::boxed::Box<DottedKey<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
    QuotedKey(::std::boxed::Box<QuotedKey<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TableChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bare_key" => {
                Ok(
                    Self::BareKey(
                        ::std::boxed::Box::new(
                            <BareKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "dotted_key" => {
                Ok(
                    Self::DottedKey(
                        ::std::boxed::Box::new(
                            <DottedKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pair" => {
                Ok(
                    Self::Pair(
                        ::std::boxed::Box::new(
                            <Pair as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "quoted_key" => {
                Ok(
                    Self::QuotedKey(
                        ::std::boxed::Box::new(
                            <QuotedKey as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TableChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BareKey(inner) => inner.span(),
            Self::DottedKey(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::QuotedKey(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableArrayElementChildren<'tree> {
    BareKey(::std::boxed::Box<BareKey<'tree>>),
    DottedKey(::std::boxed::Box<DottedKey<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
    QuotedKey(::std::boxed::Box<QuotedKey<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TableArrayElementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bare_key" => {
                Ok(
                    Self::BareKey(
                        ::std::boxed::Box::new(
                            <BareKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "dotted_key" => {
                Ok(
                    Self::DottedKey(
                        ::std::boxed::Box::new(
                            <DottedKey as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pair" => {
                Ok(
                    Self::Pair(
                        ::std::boxed::Box::new(
                            <Pair as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "quoted_key" => {
                Ok(
                    Self::QuotedKey(
                        ::std::boxed::Box::new(
                            <QuotedKey as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TableArrayElementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BareKey(inner) => inner.span(),
            Self::DottedKey(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::QuotedKey(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Array(Array<'tree>),
    Document(Document<'tree>),
    DottedKey(DottedKey<'tree>),
    Float(Float<'tree>),
    InlineTable(InlineTable<'tree>),
    Integer(Integer<'tree>),
    Pair(Pair<'tree>),
    QuotedKey(QuotedKey<'tree>),
    String(String<'tree>),
    Table(Table<'tree>),
    TableArrayElement(TableArrayElement<'tree>),
    BareKey(BareKey<'tree>),
    Boolean(Boolean<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    LocalDate(LocalDate<'tree>),
    LocalDateTime(LocalDateTime<'tree>),
    LocalTime(LocalTime<'tree>),
    OffsetDateTime(OffsetDateTime<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "array" => {
                <Array as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Array)
                    .unwrap_or(Self::Unknown(node))
            }
            "document" => {
                <Document as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Document)
                    .unwrap_or(Self::Unknown(node))
            }
            "dotted_key" => {
                <DottedKey as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DottedKey)
                    .unwrap_or(Self::Unknown(node))
            }
            "float" => {
                <Float as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Float)
                    .unwrap_or(Self::Unknown(node))
            }
            "inline_table" => {
                <InlineTable as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InlineTable)
                    .unwrap_or(Self::Unknown(node))
            }
            "integer" => {
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Integer)
                    .unwrap_or(Self::Unknown(node))
            }
            "pair" => {
                <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Pair)
                    .unwrap_or(Self::Unknown(node))
            }
            "quoted_key" => {
                <QuotedKey as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedKey)
                    .unwrap_or(Self::Unknown(node))
            }
            "string" => {
                <String as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::String)
                    .unwrap_or(Self::Unknown(node))
            }
            "table" => {
                <Table as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Table)
                    .unwrap_or(Self::Unknown(node))
            }
            "table_array_element" => {
                <TableArrayElement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TableArrayElement)
                    .unwrap_or(Self::Unknown(node))
            }
            "bare_key" => {
                <BareKey as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BareKey)
                    .unwrap_or(Self::Unknown(node))
            }
            "boolean" => {
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Boolean)
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
            "local_date" => {
                <LocalDate as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalDate)
                    .unwrap_or(Self::Unknown(node))
            }
            "local_date_time" => {
                <LocalDateTime as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalDateTime)
                    .unwrap_or(Self::Unknown(node))
            }
            "local_time" => {
                <LocalTime as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalTime)
                    .unwrap_or(Self::Unknown(node))
            }
            "offset_date_time" => {
                <OffsetDateTime as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OffsetDateTime)
                    .unwrap_or(Self::Unknown(node))
            }
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Array(inner) => inner.span(),
            Self::Document(inner) => inner.span(),
            Self::DottedKey(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::InlineTable(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::QuotedKey(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Table(inner) => inner.span(),
            Self::TableArrayElement(inner) => inner.span(),
            Self::BareKey(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::LocalDate(inner) => inner.span(),
            Self::LocalDateTime(inner) => inner.span(),
            Self::LocalTime(inner) => inner.span(),
            Self::OffsetDateTime(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
