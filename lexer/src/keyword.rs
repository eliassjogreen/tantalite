use chumsky::prelude::*;

use crate::primitive::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Fn,
    Let,
    If,
    Else,
    Persistant,
    Temporary,
    Volatile,
    Primitive(Primitive),
}

pub fn keyword_lexer<'src>() -> impl Parser<'src, &'src str, Keyword> {
    choice((
        just("fn").to(Keyword::Fn),
        just("let").to(Keyword::Let),
        just("if").to(Keyword::If),
        just("else").to(Keyword::Else),
        just("persistant").to(Keyword::Persistant),
        just("temporary").to(Keyword::Temporary),
        just("volatile").to(Keyword::Volatile),
        primitive_lexer().map(Keyword::Primitive),
    ))
}
