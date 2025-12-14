use crate::{consts::INDENT_DEDENT_SIZE, error::Error};
use logos::{FilterResult, Lexer, Logos};

/// A change in indentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Indentation {
    /// An increase in indentation.
    Indent(u16),
    /// A decrease in indentation.
    Dedent(u16),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LexerExtras {
    indent_stack: u16,
}

fn indentation(lexer: &mut Lexer<'_, Lexeme>) -> FilterResult<Indentation, Error> {
    let ws = match lexer.slice().as_bytes() {
        rn @ [b'\r', b'\n', ..] => rn.len() - 2,
        rn @ [b'\r' | b'\n', ..] => rn.len() - 1,
        _ => unreachable!(),
    };

    if !ws.is_multiple_of(INDENT_DEDENT_SIZE) {
        return FilterResult::Error(Error::ShortIndentDedent(ws));
    }

    let Ok(ws_stack) = (ws / INDENT_DEDENT_SIZE).try_into() else {
        return FilterResult::Error(Error::IndentStackOverflow);
    };

    use core::cmp::Ordering;
    match lexer.extras.indent_stack.cmp(&ws_stack) {
        Ordering::Equal => FilterResult::Skip,

        Ordering::Less => {
            lexer.extras.indent_stack = ws_stack;
            FilterResult::Emit(Indentation::Indent(ws_stack))
        }

        Ordering::Greater => {
            lexer.extras.indent_stack = ws_stack;
            FilterResult::Emit(Indentation::Dedent(ws_stack))
        }
    }
}

/// The underlying kind of a [`Token`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Logos)]
#[logos(extras = LexerExtras)]
#[logos(subpattern breaking_space = r"[\r\n]")]
#[logos(subpattern horizontal_space = r"[ ]")]
#[logos(error(Error, Error::from_lexer))]
#[logos(skip r"(?&horizontal_space)+")]
pub enum Lexeme {
    /// Keyword: `fn`
    #[token("fn")]
    Fn,
    /// Keyword: `let`
    #[token("let")]
    Let,
    /// Keyword: `return`
    #[token("return")]
    Return,
    /// Keyword: `if`
    #[token("if")]
    If,
    /// Keyword: `else`
    #[token("else")]
    Else,
    /// Keyword: `elif`
    #[token("elif")]
    Elif,
    /// Keyword: `for`
    #[token("for")]
    For,
    /// Keyword: `while`
    #[token("while")]
    While,
    /// Keyword: `break`
    #[token("break")]
    Break,
    /// Keyword: `continue`
    #[token("continue")]
    Continue,
    /// Keyword: `pass`
    #[token("pass")]
    Pass,
    /// Keyword: `in`
    #[token("in")]
    In,
    /// Keyword: `is`
    #[token("is")]
    Is,
    /// Keyword: `not`
    #[token("not")]
    Not,
    /// Keyword: `use`
    #[token("use")]
    Use,
    /// Keyword: `as`
    #[token("as")]
    As,
    /// Keyword: `pub`
    #[token("pub")]
    Pub,
    /// Keyword: `await`
    #[token("await")]
    Await,
    /// Keyword: `spawn`
    #[token("spawn")]
    Spawn,
    /// Keyword: `match`
    #[token("match")]
    Match,
    /// Keyword: `case`
    #[token("case")]
    Case,
    /// Keyword: `true`
    #[token("true")]
    True,
    /// Keyword: `false`
    #[token("false")]
    False,
    /// Keyword: `struct`
    #[token("struct")]
    Struct,
    /// Keyword: `enum`
    #[token("enum")]
    Enum,
    /// Keyword: `and`
    #[token("and")]
    And,
    /// Keyword: `or`
    #[token("or")]
    Or,

    /// A literal numeric value.
    #[regex(r"[0-9][0-9_]*")]
    Number,
    /// A literal string with builtin formatting.
    #[regex(r#"f"(\\.|[^"\\])*""#)]
    FString,
    /// A literal string.
    #[regex(r#""(\\.|[^"\\])*""#)]
    String,

    /// A name.
    #[regex(r"[\p{XID_Start}_][\p{XID_Continue}]*")]
    Identifier,

    /// An Indentation token, either indicating an
    /// [`indent`](Indentation::Indent) or a [`dedent`](Indentation::Dedent)
    #[regex(r"(?&breaking_space)+(?&horizontal_space)*", indentation)]
    IndentDedent(Indentation),

    /// Delimiter: `(`
    #[token("(")]
    OpenParen,
    /// Delimiter: `)`
    #[token(")")]
    ClosedParen,
    /// Delimiter: `{`
    #[token("{")]
    OpenBrace,
    /// Delimiter: `}`
    #[token("}")]
    ClosedBrace,
    /// Delimiter: `[`
    #[token("[")]
    OpenBracket,
    /// Delimiter: `]`
    #[token("]")]
    ClosedBracket,
    /// Delimiter: `<`
    #[token("<")]
    OpenAngle,
    /// Delimiter: `>`
    #[token(">")]
    ClosedAngle,

    /// Symbol: `:`
    #[token(":")]
    Colon,
    /// Symbol: `,`
    #[token(",")]
    Comma,
    /// Symbol: `.`
    #[token(".")]
    Dot,
    /// Symbol: `=`
    #[token("=")]
    Equals,
    /// Symbol: `+`
    #[token("+")]
    Plus,
    /// Symbol: `-`
    #[token("-")]
    Minus,
    /// Symbol: `*`
    #[token("*")]
    Star,
    /// Symbol: `/`
    #[token("/")]
    Slash,
    /// Symbol: `%`
    #[token("%")]
    Percent,
    /// Symbol: `|`
    #[token("|")]
    Pipe,
    /// Symbol: `&`
    #[token("&")]
    Amp,
    /// Symbol: `!`
    #[token("!")]
    Bang,

    /// Multi-symbol: `->`
    #[token("->")]
    Arrow,
    /// Multi-symbol: `==`
    #[token("==")]
    EqEq,
    /// Multi-symbol: `!=`
    #[token("!=")]
    Neq,
    /// Multi-symbol: `<=`
    #[token("<=")]
    LtEq,
    /// Multi-symbol: `>=`
    #[token(">=")]
    GtEq,
    /// Multi-symbol: `+=`
    #[token("+=")]
    PlusEq,
    /// Multi-symbol: `-=`
    #[token("-=")]
    MinusEq,
    /// Multi-symbol: `*=`
    #[token("*=")]
    StarEq,
    /// Multi-symbol: `/=`
    #[token("/=")]
    SlashEq,
    /// Multi-symbol: `..`
    #[token("..")]
    DoubleDot,
}

// in regards to testing, we know that `token` marked variants *will* parse correctly
// with that said, we're only going to really lock in on `regex` variants

#[cfg(test)]
mod test {
    use super::*;

    mod identifiers {
        use super::*;

        #[test]
        fn good_starts() {
            for ch in ('a'..='z').map(|ch| ch.to_string()) {
                let lexed = Lexeme::lexer(&ch).next();
                assert_eq!(lexed, Some(Ok(Lexeme::Identifier)));
            }

            for ch in ('A'..='Z').map(|ch| ch.to_string()) {
                let lexed = Lexeme::lexer(&ch).next();
                assert_eq!(lexed, Some(Ok(Lexeme::Identifier)));
            }

            let lexed = Lexeme::lexer("_").next();
            assert_eq!(lexed, Some(Ok(Lexeme::Identifier)));
        }

        #[test]
        fn bad_starts() {
            for ch in ('0'..='9').map(|ch| ch.to_string()) {
                let lexed = Lexeme::lexer(&ch).next();
                assert_ne!(lexed, Some(Ok(Lexeme::Identifier)));
            }
        }

        #[test]
        fn continuity_characters() {
            // we'll use '_' for the starting character here, just because
            // previous testing promises that it works

            let chars = ['a'..='z', 'A'..='Z', '0'..='9'];

            for cont_chars in chars {
                for ch in cont_chars.map(|ch| ch.to_string()) {
                    let lexed = Lexeme::lexer(&('_'.to_string() + &ch)).next();

                    assert_eq!(lexed, Some(Ok(Lexeme::Identifier)));
                }
            }

            let lexed = Lexeme::lexer_with_extras("__", LexerExtras::default()).next();

            assert_eq!(lexed, Some(Ok(Lexeme::Identifier)));
        }
    }

    mod numbers {
        use super::*;

        #[test]
        fn good_starts() {
            for ch in ('0'..='9').map(|ch| ch.to_string()) {
                let lexed = Lexeme::lexer_with_extras(&ch, LexerExtras::default()).next();
                assert_eq!(lexed, Some(Ok(Lexeme::Number)));
            }
        }

        #[test]
        fn bad_starts() {
            let lexed = Lexeme::lexer_with_extras("_", LexerExtras::default()).next();
            assert_ne!(lexed, Some(Ok(Lexeme::Number)));
        }

        #[test]
        fn continuity_characters() {
            // we'll use '0' for the starting character here, just because
            // previous testing promises that it works

            for ch in ('0'..='9').map(|ch| ch.to_string()) {
                let lexed =
                    Lexeme::lexer_with_extras(&('0'.to_string() + &ch), LexerExtras::default())
                        .next();

                assert_eq!(lexed, Some(Ok(Lexeme::Number)));
            }

            let lexed = Lexeme::lexer_with_extras("0_", LexerExtras::default()).next();
            assert_eq!(lexed, Some(Ok(Lexeme::Number)));
        }
    }

    mod indentation {
        use super::*;

        #[test]
        fn no_leading_horizontal() {
            let breaks = ["\r", "\n", "\r\n"];

            for source in breaks {
                let lexed = Lexeme::lexer(source).next();

                assert_eq!(lexed, None);
            }
        }

        #[test]
        fn leading_horizontal_short() {
            let breaks = ["\r   ", "\n   ", "\r\n   "];

            for source in breaks {
                let lexed = Lexeme::lexer(source).next();
                assert_eq!(lexed, Some(Err(Error::ShortIndentDedent(3))));
            }
        }

        #[test]
        fn leading_horizontal() {
            let breaks = ["\r    ", "\n    ", "\r\n    "];

            for source in breaks {
                let lexed = Lexeme::lexer(source).next();
                assert_eq!(
                    lexed,
                    Some(Ok(Lexeme::IndentDedent(Indentation::Indent(1))))
                );
            }
        }

        #[test]
        fn leading_horizontal_indent_dedent() {
            let breaks = ["\r    \r", "\n    \n", "\r\n    \r\n"];

            for source in breaks {
                let mut lexed = Lexeme::lexer(source);

                assert_eq!(
                    lexed.by_ref().next(),
                    Some(Ok(Lexeme::IndentDedent(Indentation::Indent(1))))
                );

                assert_eq!(
                    lexed.by_ref().next(),
                    Some(Ok(Lexeme::IndentDedent(Indentation::Dedent(0))))
                );
            }
        }

        #[test]
        fn overflow() {
            let spaces = " ".repeat((usize::from(u16::MAX) + 1) * INDENT_DEDENT_SIZE);
            let source = "\n".to_owned() + &spaces;

            assert_eq!(
                Lexeme::lexer(&source).next(),
                Some(Err(Error::IndentStackOverflow)),
            );
        }

        #[test]
        fn unsupported_horizontal() {
            let source = "\t";
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(
                lexed.next(),
                Some(Err(Error::UnsupportedHorizontalWhitespace('\t'))),
            );
        }
    }

    mod strings {
        use super::*;

        #[test]
        fn single_line() {
            let source = r#""foo, bar!""#;
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(lexed.next(), Some(Ok(Lexeme::String)));
        }

        #[test]
        fn multi_line() {
            let source = r#""foo,\nbar!""#;
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(lexed.next(), Some(Ok(Lexeme::String)));
        }

        #[test]
        fn escaped_quote() {
            let source = r#""foo \" bar!""#;
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(lexed.next(), Some(Ok(Lexeme::String)));
        }

        #[test]
        fn escaped_end_quote() {
            let source = r#""\""#;
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(lexed.next(), Some(Err(Error::EscapedFinalQuote)));
        }

        #[test]
        fn single_line_f() {
            let source = r#"f"foo, bar!""#;
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(lexed.next(), Some(Ok(Lexeme::FString)));
        }

        #[test]
        fn multi_line_f() {
            let source = r#"f"foo,\nbar!""#;
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(lexed.next(), Some(Ok(Lexeme::FString)));
        }

        #[test]
        fn escaped_quote_f() {
            let source = r#"f"foo \" bar!""#;
            let mut lexed = Lexeme::lexer(source);

            assert_eq!(lexed.next(), Some(Ok(Lexeme::FString)));
        }

        #[test]
        fn escaped_end_quote_f() {
            let source = r#"f"\""#;
            let mut lexed = Lexeme::lexer(source);

            // I mean you really can't avoid this here, kinda because it's like
            // just how lexers work (#1 excuse for not fixing it)
            assert_eq!(lexed.next(), Some(Ok(Lexeme::Identifier)));
            assert_eq!(lexed.next(), Some(Err(Error::EscapedFinalQuote)));
        }
    }
}
