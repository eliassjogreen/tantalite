use chumsky::prelude::*;

use crate::keyword_lexer;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Identifier<'src>(pub &'src str);

pub fn identifier_lexer<'src>() -> impl Parser<'src, &'src str, Identifier<'src>> {
    text::ident().and_is(keyword_lexer().not()).map(Identifier)
}
