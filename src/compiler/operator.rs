use phf::phf_map;
use phf::Map;

/// Operators used for expressions and declarations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operator
{
    LeftBracket,
    RightBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Walrus,
    Colon,
    ForwardSlash,
    BackSlash,
    Hyphen,
    Asterisk,
    Period,
    Comma,
    RightArrow,
    DoubleArrow,
    Pipe,
    PipeRight,
    Underscore,
    DotDot,
    DotDotDot,
    NothingCoalesce,
    Equals,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}

static OPERATOR_MAP: Map<&'static str, Operator> = phf_map!
{
    "..." => Operator::DotDotDot,

    ".." => Operator::DotDot,
    "??" => Operator::NothingCoalesce,
    "->" => Operator::RightArrow,
    "|>" => Operator::PipeRight,
    "=>" => Operator::DoubleArrow,
    ":=" => Operator::Walrus,
    "<=" => Operator::LessThanEqual,
    ">=" => Operator::GreaterThanEqual,

    "(" => Operator::LeftBracket,
    ")" => Operator::RightBracket,
    "[" => Operator::LeftSquareBracket,
    "]" => Operator::RightSquareBracket,
    ":" => Operator::Colon,
    "/" => Operator::ForwardSlash,
    "\\" => Operator::BackSlash,
    "-" => Operator::Hyphen,
    "*" => Operator::Asterisk,
    "." => Operator::Period,
    "," => Operator::Comma,
    "_" => Operator::Underscore,
    "|" => Operator::Pipe,
    "=" => Operator::Equals,
    "<" => Operator::LessThan,
    ">" => Operator::GreaterThan,
};
