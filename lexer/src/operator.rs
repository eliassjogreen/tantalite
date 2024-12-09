use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    Assign,
    Plus,
    Minus,
}

pub fn operator_lexer<'src>() -> impl Parser<'src, &'src str, Operator> {
    choice((
        just("=").to(Operator::Assign),
        just("+").to(Operator::Plus),
        just("-").to(Operator::Minus),
    ))
}
