use crate::token_stream::*;

use otterc_ast::nodes::Node;
use otterc_lexer::Lexeme;

use winnow::{error::ParserError, prelude::*, token::any};

impl<'src, O, E, P> ParserExt<'src, O, E> for P where P: Parser<TokenStream<'src>, O, E> {}
pub trait ParserExt<'src, O, E>: Parser<TokenStream<'src>, O, E> + Sized {
    fn node(self) -> impl Parser<TokenStream<'src>, Node<O>, E> {
        self.with_span().map(|(value, span)| Node::new(value, span))
    }
}

impl<'src, E> Parser<TokenStream<'src>, &'src LexToken<'src>, E> for Lexeme
where
    E: ParserError<TokenStream<'src>>,
{
    fn parse_next(&mut self, input: &mut TokenStream<'src>) -> Result<&'src LexToken<'src>, E> {
        any.verify(|tk: &LexToken| tk.lexeme() == *self)
            .parse_next(input)
    }
}
