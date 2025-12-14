use std::num::ParseFloatError;

use crate::prelude::*;

use otterc_ast::nodes::NumberLiteral;

pub fn number(input: &mut TokenStream) -> Result<NumberLiteral> {
    Lexeme::Number
        .try_map(|token: &LexToken| {
            token.source().parse().map(|value| NumberLiteral {
                value,
                is_float_literal: value.fract() == 0.,
            })
        })
        .parse_next(input)
}
