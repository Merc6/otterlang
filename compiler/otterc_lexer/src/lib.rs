//! The lexer for the otter-lang compiler

mod error;
mod token;

use logos::{Lexer as KindLexer, Logos as _};

pub use self::{
    error::{
        Error,
        Result, //
    },
    token::{
        Kind as TokenKind,
        Token, //
    },
};

/// the otter-lang lexer, reads source-text and produces `Token`s
pub struct Lexer<'src> {
    /// the heart of the lexer, produces the `Kind`s
    inner: KindLexer<'src, TokenKind>,
}

impl<'src> Lexer<'src> {
    /// constructs a new instance of `Lexer`
    #[inline]
    pub fn new<S>(source: &'src S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            inner: TokenKind::lexer(source.as_ref()),
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let span = self.inner.span();
        let source = self.inner.slice();

        Some(Token::new(
            kind,
            source,
            span.try_into().expect("span too large"),
        ))
    }
}
