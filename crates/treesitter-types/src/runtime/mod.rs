pub mod error;
pub mod from_node;
pub mod span;

pub use error::ParseError;
pub use from_node::{FromNode, LeafNode, Spanned};
pub use span::Span;

/// Grow the stack on demand to prevent overflow in deeply nested ASTs.
///
/// This is called by generated `FromNode` implementations at every recursive
/// call site. When the remaining stack space drops below the red-zone (32 KiB),
/// a new 1 MiB stack segment is allocated transparently. The overhead on the
/// happy path (enough stack left) is a single pointer comparison.
#[inline(always)]
pub fn maybe_grow_stack<R>(f: impl FnOnce() -> R) -> R {
    stacker::maybe_grow(32 * 1024, 1024 * 1024, f)
}
