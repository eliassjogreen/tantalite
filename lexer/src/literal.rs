use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'src> {
    Bool(bool),
    String(&'src str),
    Integer(i64),
    Float(f64),
}

pub fn literal_lexer<'src>() -> impl Parser<'src, &'src str, Literal<'src>> {
    choice((
        bool_lexer().map(Literal::Bool),
        string_lexer().map(Literal::String),
        integer_lexer().map(Literal::Integer),
        float_lexer().map(Literal::Float),
    ))
}

fn bool_lexer<'src>() -> impl Parser<'src, &'src str, bool> {
    choice((just("true").to(true), just("false").to(false)))
}

fn string_lexer<'src>() -> impl Parser<'src, &'src str, &'src str> {
    just('"')
        .then(any().filter(|c: &char| *c != '"').repeated())
        .then(just('"'))
        .to_slice()
}

fn integer_lexer<'src>() -> impl Parser<'src, &'src str, i64> {
    just('-')
        .or_not()
        .then(text::int(10))
        .to_slice()
        .from_str()
        .unwrapped()
}

fn float_lexer<'src>() -> impl Parser<'src, &'src str, f64> {
    just('-')
        .or_not()
        .then(text::int(10).then(just('.').then(text::int(10))))
        .to_slice()
        .from_str()
        .unwrapped()
}
