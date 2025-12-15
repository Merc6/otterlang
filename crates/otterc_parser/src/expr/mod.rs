mod call;
mod member_access;

use crate::{literal::literal, prelude::*};

use otterc_ast::Expr;
use winnow::combinator::alt;

pub fn expr(input: &mut TokenStream) -> Result<Expr> {
    alt((
        literal.noded().map(Expr::Literal),
        member_access::member_access,
        identifier.map(Expr::Identifier),
    ))
    .parse_next(input)
}
