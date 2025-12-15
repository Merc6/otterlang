mod array;
mod enum_variant;
mod r#struct;
mod wildcard;

use crate::literal::literal;
use crate::prelude::*;

use otterc_ast::nodes::Pattern;

use winnow::combinator::alt;

pub fn pattern(input: &mut TokenStream) -> Result<Pattern> {
    // try to order these such that it's sorted most-likely -> least-likely
    alt((
        enum_variant::enum_variant,
        // NOTE: wildcard should always be placed *before* identifier, as every
        // wildcard is an identifier, but not every identifier is a wildcard.
        wildcard::wildcard.value(Pattern::Wildcard),
        literal.noded().map(Pattern::Literal),
        array::array,
        r#struct::r#struct,
        identifier.map(Pattern::Identifier),
    ))
    .parse_next(input)
}
