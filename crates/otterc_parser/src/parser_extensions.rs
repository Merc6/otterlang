use crate::token_stream::*;

use otterc_ast::nodes::Node;
use otterc_lexer::Lexeme;

use winnow::{
    combinator::{opt, separated, terminated},
    error::ParserError,
    prelude::*,
    stream::{Accumulate, Range},
    token::any,
};

pub trait ParserExt<'src, O, E>: Parser<TokenStream<'src>, O, E> + Sized
where
    E: ParserError<TokenStream<'src>>,
{
    fn noded(self) -> impl Parser<TokenStream<'src>, Node<O>, E> {
        self.with_span().map(|(value, span)| Node::new(value, span))
    }

    fn comma_separated<Accumulator>(
        self,
        occurrences: impl Into<Range>,
    ) -> impl Parser<TokenStream<'src>, Accumulator, E>
    where
        Accumulator: Accumulate<O>,
    {
        terminated(
            separated(occurrences, self, Lexeme::Comma),
            opt(Lexeme::Comma),
        )
    }
}

impl<'src, O, E, P> ParserExt<'src, O, E> for P
where
    E: ParserError<TokenStream<'src>>,
    P: Parser<TokenStream<'src>, O, E>,
{
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
