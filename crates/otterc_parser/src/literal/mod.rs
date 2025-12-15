mod bool;
mod number;
mod string;
mod unit;

use crate::prelude::*;

use otterc_ast::Literal;

use winnow::combinator::alt;

pub fn literal(input: &mut TokenStream) -> Result<Literal> {
    alt((
        string::string.map(Literal::String),
        number::number.map(Literal::Number),
        bool::bool.map(Literal::Bool),
        unit::unit.value(Literal::Unit),
        // NOTE: we ignore the `Literal::None` variant, because it's in the
        // process of being phased-out, ever since enums were added.
    ))
    .parse_next(input)
}
