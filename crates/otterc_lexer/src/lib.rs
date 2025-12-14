//! The otterc-lexer. Produces `Token`s from given source-code
//!
//! # Note
//!
//! This API is completely unstable and subject to change.

#![warn(missing_docs)]

mod consts;
mod error;
mod lexeme;

use otterc_span::Span;

use logos::Lexer as Tokenizer;

pub use error::Error;
pub use lexeme::{
    Indentation,
    Lexeme, //
};

/// The [`Token`]s produced when lexing, holds the underlying type, source, and
/// span.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token<'src> {
    lexeme: Lexeme,
    source: &'src str,
    span: Span,
}

impl<'src> Token<'src> {
    /// The kind or [`Lexeme`] of `self`.
    pub fn lexeme(&self) -> Lexeme {
        self.lexeme
    }

    /// The [`source-text`](str) of `self`.
    pub fn source(&self) -> &str {
        self.source
    }

    /// The [`Span`] of `self`.
    pub fn span(&self) -> Span {
        self.span
    }
}

/// Produces [`Token`]s from [`source-text`](str).
pub struct Lexer<'src> {
    tokenizer: Tokenizer<'src, Lexeme>,
}

impl<'src> Lexer<'src> {
    /// Creates a new [`Lexer`] from [`source-text`](str).
    pub fn new(source: &'src str) -> Self {
        Lexer {
            tokenizer: Tokenizer::new(source),
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let lexeme = self.tokenizer.next()?;
        let source = self.tokenizer.slice();
        let span = Span::from(self.tokenizer.span());

        Some(lexeme.map(|lexeme| Token {
            lexeme,
            source,
            span,
        }))
    }
}
