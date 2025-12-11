use otterc_span::Span;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TokenKind {
    // Keywords
    Fn,
    Let,
    Return,
    If,
    Else,
    Elif,
    For,
    While,
    Break,
    Continue,
    Pass,
    In,
    Is,
    Not,
    Use,
    As,
    Pub,
    Await,
    Spawn,
    Match,
    Case,
    True,
    False,
    Print,
    None,
    Struct,
    Enum,
    And,
    Or,

    // Identifiers
    Identifier(String),
    UnicodeIdentifier(String),

    // Literals
    Number(String),
    StringLiteral(String),
    FString(String), // Raw f-string content like "π ≈ {result}"
    Bool(bool),

    // Structural
    Colon,
    Newline,
    Indent,
    Dedent,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,

    // Operators
    Arrow,
    Equals,
    EqEq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Pipe,
    Amp,
    Bang,

    // Assignment operators
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,

    // Range operator
    DoubleDot,

    Eof,
}

// FStringPart is now defined in the AST module

impl TokenKind {
    pub fn name(&self) -> &'static str {
        match self {
            // Keywords
            TokenKind::Fn => "fn",
            TokenKind::Let => "let",
            TokenKind::Return => "return",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Elif => "elif",
            TokenKind::For => "for",
            TokenKind::While => "while",
            TokenKind::Break => "break",
            TokenKind::Continue => "continue",
            TokenKind::Pass => "pass",
            TokenKind::In => "in",
            TokenKind::Is => "is",
            TokenKind::Not => "not",
            TokenKind::Use => "use",
            TokenKind::As => "as",
            TokenKind::Pub => "pub",
            TokenKind::Await => "await",
            TokenKind::Spawn => "spawn",
            TokenKind::Match => "match",
            TokenKind::Case => "case",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Print => "print",
            TokenKind::None => "None",
            TokenKind::Struct => "struct",
            TokenKind::Enum => "enum",
            TokenKind::And => "and",
            TokenKind::Or => "or",

            // Identifiers
            TokenKind::Identifier(_) => "identifier",
            TokenKind::UnicodeIdentifier(_) => "unicode_identifier",

            // Literals
            TokenKind::Number(_) => "number",
            TokenKind::StringLiteral(_) => "string",
            TokenKind::FString { .. } => "fstring",
            TokenKind::Bool(_) => "bool",

            // Structural
            TokenKind::Colon => ":",
            TokenKind::Newline => "newline",
            TokenKind::Indent => "indent",
            TokenKind::Dedent => "dedent",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",

            // Operators
            TokenKind::Arrow => "->",
            TokenKind::Equals => "=",
            TokenKind::EqEq => "==",
            TokenKind::Neq => "!=",
            TokenKind::Lt => "<",
            TokenKind::Gt => ">",
            TokenKind::LtEq => "<=",
            TokenKind::GtEq => ">=",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Pipe => "|",
            TokenKind::Amp => "&",
            TokenKind::Bang => "!",

            // Assignment operators
            TokenKind::PlusEq => "+=",
            TokenKind::MinusEq => "-=",
            TokenKind::StarEq => "*=",
            TokenKind::SlashEq => "/=",

            // Range operator
            TokenKind::DoubleDot => "..",

            TokenKind::Eof => "eof",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut TokenKind {
        &mut self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Fn
                | TokenKind::Let
                | TokenKind::Return
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::Elif
                | TokenKind::For
                | TokenKind::While
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Pass
                | TokenKind::In
                | TokenKind::Is
                | TokenKind::Not
                | TokenKind::Use
                | TokenKind::As
                | TokenKind::Pub
                | TokenKind::Await
                | TokenKind::Spawn
                | TokenKind::Match
                | TokenKind::Case
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Print
                | TokenKind::None
                | TokenKind::Struct
                | TokenKind::Enum
                | TokenKind::And
                | TokenKind::Or
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Number(_)
                | TokenKind::StringLiteral(_)
                | TokenKind::FString(_)
                | TokenKind::Bool(_)
                | TokenKind::None
        )
    }

    pub fn is_identifier(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Identifier(_) | TokenKind::UnicodeIdentifier(_)
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::Equals
                | TokenKind::EqEq
                | TokenKind::Neq
                | TokenKind::Lt
                | TokenKind::Gt
                | TokenKind::LtEq
                | TokenKind::GtEq
                | TokenKind::Is
                | TokenKind::Not
                | TokenKind::Arrow
                | TokenKind::Pipe
                | TokenKind::Amp
                | TokenKind::Bang
                | TokenKind::PlusEq
                | TokenKind::MinusEq
                | TokenKind::StarEq
                | TokenKind::SlashEq
                | TokenKind::DoubleDot
        )
    }

    pub fn is_structural(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::LParen
                | TokenKind::RParen
                | TokenKind::LBrace
                | TokenKind::RBrace
                | TokenKind::LBracket
                | TokenKind::RBracket
                | TokenKind::Colon
                | TokenKind::Comma
                | TokenKind::Dot
        )
    }
}
