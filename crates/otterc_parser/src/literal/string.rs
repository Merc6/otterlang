use crate::prelude::*;

pub fn string(input: &mut TokenStream) -> Result<String> {
    Lexeme::String
        .map(|token| token.source().to_owned())
        .parse_next(input)
}
