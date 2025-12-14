use crate::prelude::*;

pub fn unit(input: &mut TokenStream) -> Result<()> {
    (Lexeme::OpenParen, Lexeme::ClosedParen)
        .void()
        .parse_next(input)
}
