use super::{expr, member_access::member_access};
use crate::prelude::*;

use otterc_ast::{Expr, nodes::Node};

use winnow::combinator::delimited;

pub fn call() {}

fn call_suffix(input: &mut TokenStream) -> Result<Vec<Node<Expr>>> {
    delimited(Lexeme::OpenParen, , Lexeme::ClosedParen)
}
