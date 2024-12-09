pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod primitive;
pub mod punctuation;

use chumsky::prelude::*;

use identifier::*;
use keyword::*;
use literal::*;
use operator::*;
use punctuation::*;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Keyword(Keyword),
    Identifier(Identifier<'src>),
    Literal(Literal<'src>),
    Operator(Operator),
    Punctuation(Punctuation),
}

pub fn token_lexer<'src>() -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>> {
    choice((
        keyword_lexer().map(Token::Keyword),
        identifier_lexer().map(Token::Identifier),
        literal_lexer().map(Token::Literal),
        operator_lexer().map(Token::Operator),
        punctuation_lexer().map(Token::Punctuation),
    ))
    .map_with(|t, e| (t, e.span()))
    .padded()
    .repeated()
    .collect()
}
