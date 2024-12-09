use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Punctuation {
    Comma,
    Colon,
    Dot,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftAngleBracket,
    RightAngleBracket,
}

pub fn punctuation_lexer<'src>() -> impl Parser<'src, &'src str, Punctuation> {
    choice((
        just(",").to(Punctuation::Comma),
        just(":").to(Punctuation::Colon),
        just(".").to(Punctuation::Dot),
        just("[").to(Punctuation::LeftBracket),
        just("]").to(Punctuation::RightBracket),
        just("{").to(Punctuation::LeftBrace),
        just("}").to(Punctuation::RightBrace),
        just("(").to(Punctuation::LeftParenthesis),
        just(")").to(Punctuation::RightParenthesis),
        just("<").to(Punctuation::LeftAngleBracket),
        just(">").to(Punctuation::RightAngleBracket),
    ))
}
