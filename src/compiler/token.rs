use super::*;

/// Type of literal value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Literal
{
    Binary,
    Hexadecimal,
    Integer,
    Float,
    Double,
    Decimal,
    String,
}

/// Token used for parsing.
pub enum Token {
    Keyword(Keyword),
    Operator(Operator),
    Literal(Literal),
    Comment,
    CommentBlock,
}