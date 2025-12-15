use super::expr;
use crate::prelude::*;

use otterc_ast::{Expr, nodes::Node};
use otterc_ident::Identifier;
use winnow::combinator::{preceded, repeat};

pub fn member_access(input: &mut TokenStream) -> Result<Expr> {
    (expr.noded().boxed(), repeat(1.., accessor.noded()))
        .map(|(root, fields): (_, Vec<_>)| {
            let mut fields = fields.iter();

            let first_member = {
                let field = fields.by_ref().next().unwrap();
                let span = root.span().merge(field.span());

                Node::new(
                    Expr::Member {
                        object: root,
                        field: *field.as_ref(),
                    },
                    span,
                )
            };

            fields
                .fold(first_member, |last_member, field| {
                    let span = last_member.span().merge(field.span());

                    Node::new(
                        Expr::Member {
                            object: Box::new(last_member),
                            field: *field.as_ref(),
                        },
                        span,
                    )
                })
                .into_inner()
        })
        .parse_next(input)
}

fn accessor(input: &mut TokenStream) -> Result<Identifier> {
    preceded(Lexeme::Dot, identifier).parse_next(input)
}
