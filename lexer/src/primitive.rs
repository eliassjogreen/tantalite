use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
    Char,
    String,
}

pub fn primitive_lexer<'src>() -> impl Parser<'src, &'src str, Primitive> {
    choice((
        just("u8").to(Primitive::U8),
        just("i8").to(Primitive::I8),
        just("u16").to(Primitive::U16),
        just("i16").to(Primitive::I16),
        just("u32").to(Primitive::U32),
        just("i32").to(Primitive::I32),
        just("u64").to(Primitive::U64),
        just("i64").to(Primitive::I64),
        just("f32").to(Primitive::F32),
        just("f64").to(Primitive::F64),
        just("bool").to(Primitive::Bool),
        just("char").to(Primitive::Char),
        just("string").to(Primitive::String),
    ))
}
