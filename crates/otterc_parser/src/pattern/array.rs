use super::pattern;
use crate::prelude::*;

use otterc_ast::nodes::Pattern;

use winnow::combinator::{delimited, opt, preceded};

pub fn array(input: &mut TokenStream) -> Result<Pattern> {
    delimited(
        Lexeme::OpenBracket,
        (
            pattern.noded().comma_separated(0..),
            opt(preceded(Lexeme::DoubleDot, identifier)),
        ),
        Lexeme::ClosedBracket,
    )
    .map(|(patterns, rest)| Pattern::Array { patterns, rest })
    .parse_next(input)
}
