//! Errors that can occur when lexing

use core::{hint::unreachable_unchecked, result};

use crate::TokenKind;

use thiserror::Error as ErrDerive;

/// The errors that can occur when lexing
#[derive(Clone, Copy, Debug, Default, ErrDerive, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Error {
    /// An unknown, undefined error variant
    #[default]
    #[error("undefined error")]
    Undefined,

    /// An error variant that occurs when an unsupported whitespace character is found
    #[error("otter-lang only supports ` `, `\n`, `\r\n`; found `{0}`")]
    UnsupportedWhitespace(char),

    /// An error variant that occurs when a string is left unterminated
    #[error("string is left unterminated")]
    UnterminatedString,
}

/// A `Result` type for errors that can occur when lexing
pub type Result<T> = result::Result<T, Error>;

#[expect(
    clippy::mut_mut,
    reason = "this is the type expected by the `Kind` lexer"
)]
impl From<&mut &mut logos::Lexer<'_, TokenKind>> for Error {
    #[inline]
    fn from(value: &mut &mut logos::Lexer<'_, TokenKind>) -> Self {
        let error_slice = value.slice();

        match error_slice.chars().next() {
            // `f` is included for formatted strings
            Some('"' | 'f') => Self::UnterminatedString,
            Some(ws) if ws.is_whitespace() => Self::UnsupportedWhitespace(ws),
            Some(_) => Self::Undefined,

            // Safety:
            // The `error_slice` will always contain at least one character
            None => unsafe { unreachable_unchecked() },
        }
    }
}
