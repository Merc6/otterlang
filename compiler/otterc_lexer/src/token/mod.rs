//! Tokens that can be generated when lexing
mod kind;

use crate::Result;

use otterc_span::Span;

pub use self::{
    kind::Kind, //
};

/// The token produced when lexing
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token<'src> {
    /// The type of the token produced
    kind: Result<Kind>,
    /// The raw source of the `kind`
    source: &'src str,
    /// The coverage of the token
    span: Span,
}

impl<'src> Token<'src> {
    /// creates a new instance of `Token`, these should only be constructed internally.
    #[expect(
        clippy::single_call_fn,
        reason = "internal function only used by the lexer"
    )]
    pub(crate) const fn new(kind: Result<Kind>, source: &'src str, span: Span) -> Self {
        Self { kind, source, span }
    }
}
