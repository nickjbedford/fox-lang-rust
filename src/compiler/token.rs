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
pub enum TokenKind {
    Keyword(Keyword),
    Operator(Operator),
    Literal(Literal),
    LineComment,
    BlockComment { terminated: bool},
}

pub struct Token {
    pub kind: TokenKind,
    pub value: std::ops::Range<usize>,
    pub line: usize,
    pub column: usize,
}