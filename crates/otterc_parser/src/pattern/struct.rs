use super::pattern;
use crate::prelude::*;

use otterc_ast::nodes::Pattern;

use winnow::combinator::{delimited, opt, preceded};

pub fn r#struct(input: &mut TokenStream) -> Result<Pattern> {
    (
        identifier,
        delimited(
            Lexeme::OpenBrace,
            (identifier, opt(preceded(Lexeme::Colon, pattern.noded()))).comma_separated(1..),
            Lexeme::ClosedBrace,
        ),
    )
        .map(|(name, fields)| Pattern::Struct { name, fields })
        .parse_next(input)
}
