use crate::prelude::*;

use winnow::combinator::alt;

pub fn bool(input: &mut TokenStream) -> Result<bool> {
    alt((Lexeme::True.value(true), Lexeme::False.value(false))).parse_next(input)
}
