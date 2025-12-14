use otterc_lexer::Token;

use core::ops::Deref;
use winnow::{
    prelude::*,
    stream::{
        Location,
        TokenSlice, //
    },
};

pub type TokenStream<'src> = TokenSlice<'src, LexToken<'src>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LexToken<'src>(Token<'src>);

impl<'src> Deref for LexToken<'src> {
    type Target = Token<'src>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'src> Location for LexToken<'src> {
    fn current_token_start(&self) -> usize {
        self.span().start()
    }

    fn previous_token_end(&self) -> usize {
        self.span().start() - 1
    }
}
