use super::pattern;
use crate::prelude::*;

use otterc_ast::nodes::Pattern;
use winnow::combinator::{delimited, opt, separated_pair};

pub fn enum_variant(input: &mut TokenStream) -> Result<Pattern> {
    (
        separated_pair(identifier, Lexeme::Dot, identifier),
        opt(delimited(
            Lexeme::OpenParen,
            pattern.noded().comma_separated::<Vec<_>>(1..),
            Lexeme::ClosedParen,
        ))
        .default_value(),
    )
        .map(|((enum_name, variant), fields)| Pattern::EnumVariant {
            enum_name,
            variant,
            fields,
        })
        .parse_next(input)
}
