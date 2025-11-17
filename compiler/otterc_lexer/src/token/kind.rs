//! the lexemes that can be produced when lexing

use crate::Error;

use logos::Logos;
use num_enum::TryFromPrimitive;

/// the kind for a lexeme
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Logos, TryFromPrimitive)]
#[logos(error(Error, Error::from))]
#[logos(subpattern breaking_space = r"\r?\n")] // technically there's more but this is all otter-lang supports
#[logos(subpattern horizontal_space = r"[ ]")] // technically there's more but this is all otter-lang supports
#[logos(skip("((?&breaking_space)|(?&horizontal_space))+"))]
#[non_exhaustive]
#[repr(u8)]
pub enum Kind {
    /// Keyword: `as`
    #[token("as")]
    As,
    /// Keyword: `async`
    #[token("async")]
    Async,
    /// Keyword: `await`
    #[token("await")]
    Await,
    /// Keyword: `break`
    #[token("break")]
    Break,
    /// Keyword: `case`
    #[token("case")]
    Case,
    /// Keyword: `continue`
    #[token("continue")]
    Continue,
    /// Keyword: `def`
    #[token("def")]
    Def,
    /// Keyword: `elif`
    #[token("elif")]
    Elif,
    /// Keyword: `else`
    #[token("else")]
    Else,
    /// Keyword: `enum`
    #[token("enum")]
    Enum,
    /// Keyword: `for`
    #[token("for")]
    For,
    /// Keyword: `if`
    #[token("if")]
    If,
    /// Keyword: `in`
    #[token("in")]
    In,
    /// Keyword: `is`
    #[token("is")]
    Is,
    /// Keyword: `lambda`
    #[token("lambda")]
    Lambda,
    /// Keyword: `let`
    #[token("let")]
    Let,
    /// Keyword: `match`
    #[token("match")]
    Match,
    /// Keyword: `not`
    #[token("not")]
    Not,
    /// Keyword: `pub`
    #[token("pub")]
    Pub,
    /// Keyword: `return`
    #[token("return")]
    Return,
    /// Keyword: `spawn`
    #[token("spawn")]
    Spawn,
    /// Keyword: `struct`
    #[token("struct")]
    Struct,
    /// Keyword: `use`
    #[token("use")]
    Use,
    /// Keyword: `while`
    #[token("while")]
    While,

    /// Delimiter: `>`
    #[token(">")]
    ClosedAngle,
    /// Delimiter: `}`
    #[token("}")]
    ClosedBrace,
    /// Delimiter: `]`
    #[token("]")]
    ClosedBrack,
    /// Delimiter: `)`
    #[token(")")]
    ClosedParen,

    /// Delimiter: `<`
    #[token("<")]
    OpenAngle,
    /// Delimiter: `{`
    #[token("{")]
    OpenBrace,
    /// Delimiter: `[`
    #[token("[")]
    OpenBrack,
    /// Delimiter: `(`
    #[token("(")]
    OpenParen,

    /// Symbol: `&`
    #[token("&")]
    Ampersand,
    /// Symbol: `!`
    #[token("!")]
    Bang,
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
    Equal,
    /// Symbol: `-`
    #[token("-")]
    Minus,
    /// Symbol: `%`
    #[token("%")]
    Percent,
    /// Symbol: `|`
    #[token("|")]
    Pipe,
    /// Symbol: `+`
    #[token("+")]
    Plus,
    /// Symbol: `/`
    #[token("/")]
    Slash,
    /// Symbol: `*`
    #[token("*")]
    Star,

    /// Wide-symbol: `!=`
    #[token("!=")]
    BangEqual,
    /// Wide-symbol: `..`
    #[token("..")]
    DotDot,
    /// Wide-symbol: `..=`
    #[token("..=")]
    DotDotEqual,
    /// Wide-symbol: `==`
    #[token("==")]
    EqualEqual,
    /// Wide-symbol: `>=`
    #[token(">=")]
    GreaterThanEqual,
    /// Wide-symbol: `<=`
    #[token("<=")]
    LessThanEqual,
    /// Wide-symbol: `-=`
    #[token("-=")]
    MinusEqual,
    /// Wide-symbol: `+=`
    #[token("+=")]
    PlusEqual,
    /// Wide-symbol: `/=`
    #[token("/=")]
    SlashEqual,
    /// Wide-symbol: `*=`
    #[token("*=")]
    StarEqual,
    /// Wide-symbol: `->`
    #[token("->")]
    ThinRightArrow,

    /// falsy boolean literal.
    #[token("false")]
    False,
    /// float literal.
    #[regex(r"[0-9](_*[0-9])*\.(_*[0-9])*")]
    Float,
    /// string literal with formatting.
    #[regex(r#"f"(\\.|[^(?&breaking_space)"])*""#)]
    FormattingString,
    /// integer literal.
    #[regex(r"[0-9](_*[0-9])*")]
    Integer,
    /// string literal.
    #[regex(r#""(\\.|[^(?&breaking_space)"])*""#)]
    String,
    /// truthy boolean literal.
    #[token("true")]
    True,

    /// a comment in the source code.
    #[regex(r"#[^(?&breaking_space)]*")]
    Comment,
    /// an identifier.
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    /// indentation, more specifically, the leading indentation - if any - following a line-break.
    #[regex(r"(?&breaking_space)(?&horizontal_space)+")]
    Indentation,
}

impl Kind {
    /// returns whether or not `self` is a boolean.
    #[inline]
    #[must_use]
    pub fn is_boolean(self) -> bool {
        self == Self::True || self == Self::False
    }

    /// returns whether or not `self` is a delimiter.
    #[inline]
    #[must_use]
    pub fn is_delimiter(self) -> bool {
        Self::ClosedAngle <= self && self <= Self::OpenParen
    }

    /// returns whether or not `self` is an identifier.
    ///
    /// # Note
    ///
    /// this includes keywords.
    #[inline]
    #[must_use]
    pub fn is_identifier(self) -> bool {
        self.is_keyword() || self == Self::Identifier
    }

    /// returns whether or not `self` is a keyword.
    #[inline]
    #[must_use]
    pub fn is_keyword(self) -> bool {
        Self::As <= self && self <= Self::While
    }

    /// returns whether or not `self` is a literal.
    #[inline]
    #[must_use]
    pub fn is_literal(self) -> bool {
        self.is_string() || self.is_boolean() || self.is_number()
    }

    /// returns whether or not `self` is an integer OR float.
    #[inline]
    #[must_use]
    pub fn is_number(self) -> bool {
        self == Self::Integer || self == Self::Float
    }

    /// returns whether or not `self` is a string OR formatted-string.
    #[inline]
    #[must_use]
    pub fn is_string(self) -> bool {
        self == Self::String || self == Self::FormattingString
    }

    /// returns whether or not `self` is a thin-symbol OR wide-symbol.
    #[inline]
    #[must_use]
    pub fn is_symbol(self) -> bool {
        self.is_thin_symbol() || self.is_wide_symbol()
    }

    /// returns whether or not `self` is a thin-symbol.
    #[inline]
    #[must_use]
    pub fn is_thin_symbol(self) -> bool {
        Self::Ampersand <= self && self <= Self::Star
    }

    /// returns whether or not `self` is a wide-symbol.
    #[inline]
    #[must_use]
    pub fn is_wide_symbol(self) -> bool {
        Self::BangEqual <= self && self <= Self::ThinRightArrow
    }
}
