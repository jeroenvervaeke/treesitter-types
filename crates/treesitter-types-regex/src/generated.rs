#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alternation<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Term<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Alternation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alternation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Term as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Alternation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnonymousCapturingGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousCapturingGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anonymous_capturing_group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for AnonymousCapturingGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackreferenceEscape<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: GroupName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BackreferenceEscape<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "backreference_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <GroupName as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <GroupName as ::treesitter_types::FromNode>::from_node(
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
                    <GroupName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BackreferenceEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterClass<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CharacterClassChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterClass<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character_class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <CharacterClassChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CharacterClass<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterClassEscape<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<CharacterClassEscapeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterClassEscape<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character_class_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <CharacterClassEscapeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CharacterClassEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassRange<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassRangeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassRange<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_range");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassRangeChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassRange<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlEscape<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ControlEscape<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "control_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ControlEscape<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ControlEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountQuantifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CountQuantifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CountQuantifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "count_quantifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <CountQuantifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CountQuantifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flags<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Flags<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "flags");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Flags<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Flags<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineFlagsGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InlineFlagsGroupChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InlineFlagsGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inline_flags_group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <InlineFlagsGroupChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InlineFlagsGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookaroundAssertion<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LookaroundAssertion<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lookaround_assertion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for LookaroundAssertion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedCapturingGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NamedCapturingGroupChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedCapturingGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_capturing_group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <NamedCapturingGroupChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedCapturingGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedGroupBackreference<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: GroupName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedGroupBackreference<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_group_backreference");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <GroupName as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <GroupName as ::treesitter_types::FromNode>::from_node(
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
                    <GroupName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedGroupBackreference<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonCapturingGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NonCapturingGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "non_capturing_group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for NonCapturingGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneOrMore<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Lazy<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OneOrMore<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "one_or_more");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Lazy as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for OneOrMore<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Lazy<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Optional<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Lazy as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Optional<'_> {
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
pub struct PosixCharacterClass<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PosixClassName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PosixCharacterClass<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "posix_character_class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <PosixClassName as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <PosixClassName as ::treesitter_types::FromNode>::from_node(
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
                    <PosixClassName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PosixCharacterClass<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PosixClassName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PosixClassName<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "posix_class_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PosixClassName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PosixClassName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartAssertion<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StartAssertion<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "start_assertion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StartAssertion<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StartAssertion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TermChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Term<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "term");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TermChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Term<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnicodeCharacterEscape<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnicodeCharacterEscape<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unicode_character_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UnicodeCharacterEscape<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UnicodeCharacterEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnicodePropertyValueExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UnicodePropertyValueExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnicodePropertyValueExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unicode_property_value_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            ::treesitter_types::runtime::maybe_grow_stack(|| <UnicodePropertyValueExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for UnicodePropertyValueExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroOrMore<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Lazy<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ZeroOrMore<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "zero_or_more");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Lazy as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ZeroOrMore<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyCharacter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnyCharacter<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "any_character");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AnyCharacter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AnyCharacter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryAssertion<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BoundaryAssertion<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boundary_assertion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BoundaryAssertion<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BoundaryAssertion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassCharacter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassCharacter<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_character");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ClassCharacter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ClassCharacter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlLetterEscape<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ControlLetterEscape<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "control_letter_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ControlLetterEscape<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ControlLetterEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecimalDigits<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecimalDigits<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "decimal_digits");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DecimalDigits<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DecimalDigits<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecimalEscape<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecimalEscape<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "decimal_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DecimalEscape<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DecimalEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndAssertion<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EndAssertion<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "end_assertion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for EndAssertion<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for EndAssertion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GroupName<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "group_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for GroupName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for GroupName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityEscape<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IdentityEscape<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "identity_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IdentityEscape<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IdentityEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lazy<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Lazy<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lazy");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Lazy<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Lazy<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonBoundaryAssertion<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NonBoundaryAssertion<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "non_boundary_assertion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NonBoundaryAssertion<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NonBoundaryAssertion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternCharacter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternCharacter<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pattern_character");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PatternCharacter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PatternCharacter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnicodePropertyName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnicodePropertyName<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unicode_property_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UnicodePropertyName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UnicodePropertyName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnicodePropertyValue<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnicodePropertyValue<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unicode_property_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UnicodePropertyValue<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UnicodePropertyValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterClassChildren<'tree> {
    CharacterClassEscape(::std::boxed::Box<CharacterClassEscape<'tree>>),
    ClassCharacter(::std::boxed::Box<ClassCharacter<'tree>>),
    ClassRange(::std::boxed::Box<ClassRange<'tree>>),
    ControlEscape(::std::boxed::Box<ControlEscape<'tree>>),
    ControlLetterEscape(::std::boxed::Box<ControlLetterEscape<'tree>>),
    IdentityEscape(::std::boxed::Box<IdentityEscape<'tree>>),
    PosixCharacterClass(::std::boxed::Box<PosixCharacterClass<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterClassChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "character_class_escape" => Ok(Self::CharacterClassEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharacterClassEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class_character" => Ok(Self::ClassCharacter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassCharacter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class_range" => Ok(Self::ClassRange(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassRange as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "control_escape" => Ok(Self::ControlEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ControlEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "control_letter_escape" => Ok(Self::ControlLetterEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ControlLetterEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identity_escape" => Ok(Self::IdentityEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IdentityEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "posix_character_class" => Ok(Self::PosixCharacterClass(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PosixCharacterClass as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharacterClassChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CharacterClassEscape(inner) => inner.span(),
            Self::ClassCharacter(inner) => inner.span(),
            Self::ClassRange(inner) => inner.span(),
            Self::ControlEscape(inner) => inner.span(),
            Self::ControlLetterEscape(inner) => inner.span(),
            Self::IdentityEscape(inner) => inner.span(),
            Self::PosixCharacterClass(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharacterClassEscapeChildren<'tree> {
    UnicodeCharacterEscape(::std::boxed::Box<UnicodeCharacterEscape<'tree>>),
    UnicodePropertyValueExpression(::std::boxed::Box<UnicodePropertyValueExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterClassEscapeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "unicode_character_escape" => Ok(Self::UnicodeCharacterEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnicodeCharacterEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unicode_property_value_expression" => Ok(Self::UnicodePropertyValueExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnicodePropertyValueExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharacterClassEscapeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UnicodeCharacterEscape(inner) => inner.span(),
            Self::UnicodePropertyValueExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassRangeChildren<'tree> {
    CharacterClassEscape(::std::boxed::Box<CharacterClassEscape<'tree>>),
    ClassCharacter(::std::boxed::Box<ClassCharacter<'tree>>),
    ControlEscape(::std::boxed::Box<ControlEscape<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassRangeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "character_class_escape" => Ok(Self::CharacterClassEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharacterClassEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class_character" => Ok(Self::ClassCharacter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassCharacter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "control_escape" => Ok(Self::ControlEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ControlEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassRangeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CharacterClassEscape(inner) => inner.span(),
            Self::ClassCharacter(inner) => inner.span(),
            Self::ControlEscape(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CountQuantifierChildren<'tree> {
    DecimalDigits(::std::boxed::Box<DecimalDigits<'tree>>),
    Lazy(::std::boxed::Box<Lazy<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CountQuantifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "decimal_digits" => Ok(Self::DecimalDigits(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DecimalDigits as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lazy" => Ok(Self::Lazy(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lazy as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CountQuantifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DecimalDigits(inner) => inner.span(),
            Self::Lazy(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineFlagsGroupChildren<'tree> {
    Flags(::std::boxed::Box<Flags<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InlineFlagsGroupChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "flags" => Ok(Self::Flags(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Flags as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pattern" => Ok(Self::Pattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InlineFlagsGroupChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Flags(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamedCapturingGroupChildren<'tree> {
    GroupName(::std::boxed::Box<GroupName<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedCapturingGroupChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "group_name" => Ok(Self::GroupName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GroupName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pattern" => Ok(Self::Pattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamedCapturingGroupChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GroupName(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternChildren<'tree> {
    Alternation(::std::boxed::Box<Alternation<'tree>>),
    Term(::std::boxed::Box<Term<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alternation" => Ok(Self::Alternation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Alternation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "term" => Ok(Self::Term(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Term as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Alternation(inner) => inner.span(),
            Self::Term(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermChildren<'tree> {
    AnonymousCapturingGroup(::std::boxed::Box<AnonymousCapturingGroup<'tree>>),
    AnyCharacter(::std::boxed::Box<AnyCharacter<'tree>>),
    BackreferenceEscape(::std::boxed::Box<BackreferenceEscape<'tree>>),
    BoundaryAssertion(::std::boxed::Box<BoundaryAssertion<'tree>>),
    CharacterClass(::std::boxed::Box<CharacterClass<'tree>>),
    CharacterClassEscape(::std::boxed::Box<CharacterClassEscape<'tree>>),
    ControlEscape(::std::boxed::Box<ControlEscape<'tree>>),
    ControlLetterEscape(::std::boxed::Box<ControlLetterEscape<'tree>>),
    CountQuantifier(::std::boxed::Box<CountQuantifier<'tree>>),
    DecimalEscape(::std::boxed::Box<DecimalEscape<'tree>>),
    EndAssertion(::std::boxed::Box<EndAssertion<'tree>>),
    IdentityEscape(::std::boxed::Box<IdentityEscape<'tree>>),
    InlineFlagsGroup(::std::boxed::Box<InlineFlagsGroup<'tree>>),
    LookaroundAssertion(::std::boxed::Box<LookaroundAssertion<'tree>>),
    NamedCapturingGroup(::std::boxed::Box<NamedCapturingGroup<'tree>>),
    NamedGroupBackreference(::std::boxed::Box<NamedGroupBackreference<'tree>>),
    NonBoundaryAssertion(::std::boxed::Box<NonBoundaryAssertion<'tree>>),
    NonCapturingGroup(::std::boxed::Box<NonCapturingGroup<'tree>>),
    OneOrMore(::std::boxed::Box<OneOrMore<'tree>>),
    Optional(::std::boxed::Box<Optional<'tree>>),
    PatternCharacter(::std::boxed::Box<PatternCharacter<'tree>>),
    PosixCharacterClass(::std::boxed::Box<PosixCharacterClass<'tree>>),
    StartAssertion(::std::boxed::Box<StartAssertion<'tree>>),
    ZeroOrMore(::std::boxed::Box<ZeroOrMore<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TermChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "anonymous_capturing_group" => Ok(Self::AnonymousCapturingGroup(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnonymousCapturingGroup as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "any_character" => Ok(Self::AnyCharacter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AnyCharacter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "backreference_escape" => Ok(Self::BackreferenceEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BackreferenceEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "boundary_assertion" => Ok(Self::BoundaryAssertion(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BoundaryAssertion as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "character_class" => Ok(Self::CharacterClass(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharacterClass as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "character_class_escape" => Ok(Self::CharacterClassEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharacterClassEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "control_escape" => Ok(Self::ControlEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ControlEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "control_letter_escape" => Ok(Self::ControlLetterEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ControlLetterEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "count_quantifier" => Ok(Self::CountQuantifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CountQuantifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "decimal_escape" => Ok(Self::DecimalEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DecimalEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "end_assertion" => Ok(Self::EndAssertion(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EndAssertion as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identity_escape" => Ok(Self::IdentityEscape(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IdentityEscape as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "inline_flags_group" => Ok(Self::InlineFlagsGroup(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InlineFlagsGroup as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lookaround_assertion" => Ok(Self::LookaroundAssertion(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LookaroundAssertion as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "named_capturing_group" => Ok(Self::NamedCapturingGroup(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NamedCapturingGroup as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "named_group_backreference" => Ok(Self::NamedGroupBackreference(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NamedGroupBackreference as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "non_boundary_assertion" => Ok(Self::NonBoundaryAssertion(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NonBoundaryAssertion as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "non_capturing_group" => Ok(Self::NonCapturingGroup(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NonCapturingGroup as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "one_or_more" => Ok(Self::OneOrMore(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OneOrMore as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "optional" => Ok(Self::Optional(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Optional as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pattern_character" => Ok(Self::PatternCharacter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PatternCharacter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "posix_character_class" => Ok(Self::PosixCharacterClass(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PosixCharacterClass as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "start_assertion" => Ok(Self::StartAssertion(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StartAssertion as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "zero_or_more" => Ok(Self::ZeroOrMore(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ZeroOrMore as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TermChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnonymousCapturingGroup(inner) => inner.span(),
            Self::AnyCharacter(inner) => inner.span(),
            Self::BackreferenceEscape(inner) => inner.span(),
            Self::BoundaryAssertion(inner) => inner.span(),
            Self::CharacterClass(inner) => inner.span(),
            Self::CharacterClassEscape(inner) => inner.span(),
            Self::ControlEscape(inner) => inner.span(),
            Self::ControlLetterEscape(inner) => inner.span(),
            Self::CountQuantifier(inner) => inner.span(),
            Self::DecimalEscape(inner) => inner.span(),
            Self::EndAssertion(inner) => inner.span(),
            Self::IdentityEscape(inner) => inner.span(),
            Self::InlineFlagsGroup(inner) => inner.span(),
            Self::LookaroundAssertion(inner) => inner.span(),
            Self::NamedCapturingGroup(inner) => inner.span(),
            Self::NamedGroupBackreference(inner) => inner.span(),
            Self::NonBoundaryAssertion(inner) => inner.span(),
            Self::NonCapturingGroup(inner) => inner.span(),
            Self::OneOrMore(inner) => inner.span(),
            Self::Optional(inner) => inner.span(),
            Self::PatternCharacter(inner) => inner.span(),
            Self::PosixCharacterClass(inner) => inner.span(),
            Self::StartAssertion(inner) => inner.span(),
            Self::ZeroOrMore(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnicodePropertyValueExpressionChildren<'tree> {
    UnicodePropertyName(::std::boxed::Box<UnicodePropertyName<'tree>>),
    UnicodePropertyValue(::std::boxed::Box<UnicodePropertyValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnicodePropertyValueExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "unicode_property_name" => Ok(Self::UnicodePropertyName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnicodePropertyName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unicode_property_value" => Ok(Self::UnicodePropertyValue(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnicodePropertyValue as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnicodePropertyValueExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UnicodePropertyName(inner) => inner.span(),
            Self::UnicodePropertyValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Alternation(Alternation<'tree>),
    AnonymousCapturingGroup(AnonymousCapturingGroup<'tree>),
    BackreferenceEscape(BackreferenceEscape<'tree>),
    CharacterClass(CharacterClass<'tree>),
    CharacterClassEscape(CharacterClassEscape<'tree>),
    ClassRange(ClassRange<'tree>),
    ControlEscape(ControlEscape<'tree>),
    CountQuantifier(CountQuantifier<'tree>),
    Flags(Flags<'tree>),
    InlineFlagsGroup(InlineFlagsGroup<'tree>),
    LookaroundAssertion(LookaroundAssertion<'tree>),
    NamedCapturingGroup(NamedCapturingGroup<'tree>),
    NamedGroupBackreference(NamedGroupBackreference<'tree>),
    NonCapturingGroup(NonCapturingGroup<'tree>),
    OneOrMore(OneOrMore<'tree>),
    Optional(Optional<'tree>),
    Pattern(Pattern<'tree>),
    PosixCharacterClass(PosixCharacterClass<'tree>),
    PosixClassName(PosixClassName<'tree>),
    StartAssertion(StartAssertion<'tree>),
    Term(Term<'tree>),
    UnicodeCharacterEscape(UnicodeCharacterEscape<'tree>),
    UnicodePropertyValueExpression(UnicodePropertyValueExpression<'tree>),
    ZeroOrMore(ZeroOrMore<'tree>),
    AnyCharacter(AnyCharacter<'tree>),
    BoundaryAssertion(BoundaryAssertion<'tree>),
    ClassCharacter(ClassCharacter<'tree>),
    ControlLetterEscape(ControlLetterEscape<'tree>),
    DecimalDigits(DecimalDigits<'tree>),
    DecimalEscape(DecimalEscape<'tree>),
    EndAssertion(EndAssertion<'tree>),
    GroupName(GroupName<'tree>),
    IdentityEscape(IdentityEscape<'tree>),
    Lazy(Lazy<'tree>),
    NonBoundaryAssertion(NonBoundaryAssertion<'tree>),
    PatternCharacter(PatternCharacter<'tree>),
    UnicodePropertyName(UnicodePropertyName<'tree>),
    UnicodePropertyValue(UnicodePropertyValue<'tree>),
    Unknown(::treesitter_types::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::treesitter_types::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "alternation" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Alternation as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Alternation)
            .unwrap_or(Self::Unknown(node)),
            "anonymous_capturing_group" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AnonymousCapturingGroup as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AnonymousCapturingGroup)
            .unwrap_or(Self::Unknown(node)),
            "backreference_escape" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BackreferenceEscape as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BackreferenceEscape)
            .unwrap_or(Self::Unknown(node)),
            "character_class" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CharacterClass as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CharacterClass)
            .unwrap_or(Self::Unknown(node)),
            "character_class_escape" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CharacterClassEscape as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CharacterClassEscape)
            .unwrap_or(Self::Unknown(node)),
            "class_range" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClassRange as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassRange)
            .unwrap_or(Self::Unknown(node)),
            "control_escape" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ControlEscape as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ControlEscape)
            .unwrap_or(Self::Unknown(node)),
            "count_quantifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CountQuantifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CountQuantifier)
            .unwrap_or(Self::Unknown(node)),
            "flags" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Flags as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Flags)
            .unwrap_or(Self::Unknown(node)),
            "inline_flags_group" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InlineFlagsGroup as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InlineFlagsGroup)
            .unwrap_or(Self::Unknown(node)),
            "lookaround_assertion" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LookaroundAssertion as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LookaroundAssertion)
            .unwrap_or(Self::Unknown(node)),
            "named_capturing_group" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NamedCapturingGroup as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NamedCapturingGroup)
            .unwrap_or(Self::Unknown(node)),
            "named_group_backreference" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NamedGroupBackreference as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NamedGroupBackreference)
            .unwrap_or(Self::Unknown(node)),
            "non_capturing_group" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NonCapturingGroup as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NonCapturingGroup)
            .unwrap_or(Self::Unknown(node)),
            "one_or_more" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OneOrMore as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OneOrMore)
            .unwrap_or(Self::Unknown(node)),
            "optional" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Optional as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Optional)
            .unwrap_or(Self::Unknown(node)),
            "pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pattern)
            .unwrap_or(Self::Unknown(node)),
            "posix_character_class" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PosixCharacterClass as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PosixCharacterClass)
            .unwrap_or(Self::Unknown(node)),
            "posix_class_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PosixClassName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PosixClassName)
            .unwrap_or(Self::Unknown(node)),
            "start_assertion" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StartAssertion as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StartAssertion)
            .unwrap_or(Self::Unknown(node)),
            "term" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Term as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Term)
            .unwrap_or(Self::Unknown(node)),
            "unicode_character_escape" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnicodeCharacterEscape as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnicodeCharacterEscape)
            .unwrap_or(Self::Unknown(node)),
            "unicode_property_value_expression" => {
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnicodePropertyValueExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })
                .map(Self::UnicodePropertyValueExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "zero_or_more" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ZeroOrMore as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ZeroOrMore)
            .unwrap_or(Self::Unknown(node)),
            "any_character" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AnyCharacter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AnyCharacter)
            .unwrap_or(Self::Unknown(node)),
            "boundary_assertion" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BoundaryAssertion as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BoundaryAssertion)
            .unwrap_or(Self::Unknown(node)),
            "class_character" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClassCharacter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassCharacter)
            .unwrap_or(Self::Unknown(node)),
            "control_letter_escape" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ControlLetterEscape as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ControlLetterEscape)
            .unwrap_or(Self::Unknown(node)),
            "decimal_digits" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DecimalDigits as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DecimalDigits)
            .unwrap_or(Self::Unknown(node)),
            "decimal_escape" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DecimalEscape as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DecimalEscape)
            .unwrap_or(Self::Unknown(node)),
            "end_assertion" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EndAssertion as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EndAssertion)
            .unwrap_or(Self::Unknown(node)),
            "group_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GroupName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GroupName)
            .unwrap_or(Self::Unknown(node)),
            "identity_escape" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IdentityEscape as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IdentityEscape)
            .unwrap_or(Self::Unknown(node)),
            "lazy" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Lazy as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Lazy)
            .unwrap_or(Self::Unknown(node)),
            "non_boundary_assertion" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NonBoundaryAssertion as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NonBoundaryAssertion)
            .unwrap_or(Self::Unknown(node)),
            "pattern_character" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PatternCharacter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PatternCharacter)
            .unwrap_or(Self::Unknown(node)),
            "unicode_property_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnicodePropertyName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnicodePropertyName)
            .unwrap_or(Self::Unknown(node)),
            "unicode_property_value" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnicodePropertyValue as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnicodePropertyValue)
            .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Alternation(inner) => inner.span(),
            Self::AnonymousCapturingGroup(inner) => inner.span(),
            Self::BackreferenceEscape(inner) => inner.span(),
            Self::CharacterClass(inner) => inner.span(),
            Self::CharacterClassEscape(inner) => inner.span(),
            Self::ClassRange(inner) => inner.span(),
            Self::ControlEscape(inner) => inner.span(),
            Self::CountQuantifier(inner) => inner.span(),
            Self::Flags(inner) => inner.span(),
            Self::InlineFlagsGroup(inner) => inner.span(),
            Self::LookaroundAssertion(inner) => inner.span(),
            Self::NamedCapturingGroup(inner) => inner.span(),
            Self::NamedGroupBackreference(inner) => inner.span(),
            Self::NonCapturingGroup(inner) => inner.span(),
            Self::OneOrMore(inner) => inner.span(),
            Self::Optional(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::PosixCharacterClass(inner) => inner.span(),
            Self::PosixClassName(inner) => inner.span(),
            Self::StartAssertion(inner) => inner.span(),
            Self::Term(inner) => inner.span(),
            Self::UnicodeCharacterEscape(inner) => inner.span(),
            Self::UnicodePropertyValueExpression(inner) => inner.span(),
            Self::ZeroOrMore(inner) => inner.span(),
            Self::AnyCharacter(inner) => inner.span(),
            Self::BoundaryAssertion(inner) => inner.span(),
            Self::ClassCharacter(inner) => inner.span(),
            Self::ControlLetterEscape(inner) => inner.span(),
            Self::DecimalDigits(inner) => inner.span(),
            Self::DecimalEscape(inner) => inner.span(),
            Self::EndAssertion(inner) => inner.span(),
            Self::GroupName(inner) => inner.span(),
            Self::IdentityEscape(inner) => inner.span(),
            Self::Lazy(inner) => inner.span(),
            Self::NonBoundaryAssertion(inner) => inner.span(),
            Self::PatternCharacter(inner) => inner.span(),
            Self::UnicodePropertyName(inner) => inner.span(),
            Self::UnicodePropertyValue(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
