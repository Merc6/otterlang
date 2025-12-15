mod expr;
mod literal;
mod parser_extensions;
mod pattern;
mod token_stream;

mod prelude {
    use super::*;

    use otterc_ident::Identifier;

    pub use parser_extensions::*;
    pub use token_stream::*;

    pub use otterc_lexer::Lexeme;
    pub use winnow::{Result, prelude::*};

    pub fn identifier(input: &mut TokenStream) -> Result<Identifier> {
        Lexeme::Identifier
            .map(|token: &LexToken| Identifier::new(token.source()))
            .parse_next(input)
    }
}
