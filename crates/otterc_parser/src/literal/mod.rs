mod bool;
mod number;
mod string;
mod unit;

use crate::prelude::*;

use otterc_ast::Literal;

use winnow::combinator::alt;

fn literal(input: &mut TokenStream) -> Result<Literal> {
    alt((
        string::string.map(Literal::String),
        number::number.map(Literal::Number),
        bool::bool.map(Literal::Bool),
        unit::unit.value(Literal::Unit),
    ))
    .parse_next(input)
}
