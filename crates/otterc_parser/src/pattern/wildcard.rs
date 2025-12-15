use crate::prelude::*;

pub fn wildcard(input: &mut TokenStream) -> Result<()> {
    Lexeme::Identifier
        .verify(|token: &LexToken| matches!(token.source().as_bytes(), [b'_']))
        .void()
        .parse_next(input)
}
