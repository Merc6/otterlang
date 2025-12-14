mod error;

use std::ops::Deref;

use otterc_ast::nodes::Node;
use otterc_ident::Identifier;
use otterc_lexer::{Lexeme, Lexer, Token};

use winnow::{
    Result,
    combinator::{self, opt},
    prelude::*,
    stream::{Location, TokenSlice},
    token::any,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LexToken<'src>(Token<'src>);

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

type TokenStream<'src> = TokenSlice<'src, LexToken<'src>>;

fn identifier<'src>(input: &mut TokenStream<'src>) -> Result<Identifier> {
    any.verify(|tk: &LexToken| matches!(tk.lexeme(), Lexeme::Identifier))
        .map(|tk: &LexToken| Identifier::new(tk.source()))
        .parse_next(input)
}

fn boolean<'src>(input: &mut TokenStream<'src>) -> Result<bool> {
    combinator::alt((keyword::r#true.value(true), keyword::r#false.value(false))).parse_next(input)
}

#[test]
fn bools() {
    let source = "false";
    let lexed = Lexer::new(source)
        .map(Result::unwrap)
        .map(LexToken)
        .collect::<Vec<_>>();

    let mut input = TokenStream::new(&lexed);

    let parsed = boolean
        .parse_next(&mut input)
        .expect("`true` should be an boolean");

    assert_eq!(false, parsed);
}

mod keyword {
    use super::*;

    macro_rules! unit_lexeme {
        ($($fn_name:ident => $lexeme_name:ident),* $(,)?) => {
            $(
                pub fn $fn_name<'src>(input: &mut TokenStream<'src>) -> Result<()> {
                    any.verify(|tk: &LexToken| matches!(tk.lexeme(), Lexeme::$lexeme_name))
                        .parse_next(input)?;

                    Ok(())
                }
            )*
        };
    }

    unit_lexeme! {
        r#fn       => Fn,
        r#let      => Let,
        r#return   => Return,
        r#if       => If,
        r#else     => Else,
        elif       => Elif,
        r#for      => For,
        r#while    => While,
        r#break    => Break,
        r#continue => Continue,
        pass       => Pass,
        r#in       => In,
        is         => Is,
        not        => Not,
        r#use      => Use,
        r#as       => As,
        r#pub      => Pub,
        r#await    => Await,
        spawn      => Spawn,
        r#match    => Match,
        case       => Case,
        r#true     => True,
        r#false    => False,
        r#struct   => Struct,
        r#enum     => Enum,
        and        => And,
        or         => Or,
    }
}
