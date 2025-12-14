use logos::Lexer;

use crate::{Lexeme, consts::INDENT_DEDENT_SIZE};

/// Errors that can occur while lexing.
#[derive(Clone, Debug, Default, thiserror::Error, PartialEq, Eq, Hash)]
pub enum Error {
    /// An error that occurs as a result of an indent level being either too
    /// short, or too long.
    #[error("Expected {0} to be a multiple of {INDENT_DEDENT_SIZE}")]
    ShortIndentDedent(usize),

    /// An error that occurs as a result of the indentation stack overflowing
    /// while lexing.
    #[error(
        "Indentation stack overflowed, you may only have {} indentation levels, ({} columns)",
        u16::MAX,
        u16::MAX as usize * INDENT_DEDENT_SIZE
    )]
    IndentStackOverflow,

    /// An error that occurs as a result of the final quote in a string being
    /// escaped.
    #[error("The right-most delimiting quote was escaped")]
    EscapedFinalQuote,

    /// An error that occurs as a result of an unsupported horizontal-
    /// whitespace character being lexed.
    #[error("Otter-lang only supports `space` as its horizontal-whitespace (got {0})")]
    UnsupportedHorizontalWhitespace(char),

    /// An error that occurs as a result of something going wrong, if this
    /// variant occurs, file an issue in the Github repo.
    #[default]
    #[error("An unknown error occurred while lexing")]
    Unknown,
}

impl Error {
    pub(crate) fn from_lexer(lexer: &mut Lexer<Lexeme>) -> Self {
        match lexer.slice().as_bytes() {
            [b'"', .., b'\\', b'"'] => Self::EscapedFinalQuote,
            [b'\t', ..] => Self::UnsupportedHorizontalWhitespace('\t'),
            _ => Self::default(),
        }
    }
}
