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
    Asterisk,
    Period,
    Comma,
    RightArrow,
    DoubleArrow,
    BitOr,
    PipeRight,
    Underscore,
    DotDot,
    Ellipsis,
    NothingCoalesce,
    Assign,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Tilde,
    Plus,
    Minus,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    ForwardSlashEqual,
    Exclamation,
    And,
    BitAnd,
    Caret,
    Percent,
    Equals,
    NotEquals,
    Or,
    Question,
}

static OPERATOR_MAP: Map<&'static str, Operator> = phf_map!
{
    "..." => Operator::Ellipsis,

    ".." => Operator::DotDot,
    "??" => Operator::NothingCoalesce,
    "->" => Operator::RightArrow,
    "|>" => Operator::PipeRight,
    "=>" => Operator::DoubleArrow,
    ":=" => Operator::Walrus,
    "<=" => Operator::LessThanEqual,
    ">=" => Operator::GreaterThanEqual,
    "+=" => Operator::PlusEqual,
    "-=" => Operator::MinusEqual,
    "*=" => Operator::AsteriskEqual,
    "/=" => Operator::ForwardSlashEqual,
    "!=" => Operator::NotEquals,
    "==" => Operator::Equals,
    "||" => Operator::Or,
    "&&" => Operator::And,

    "(" => Operator::LeftBracket,
    ")" => Operator::RightBracket,
    "[" => Operator::LeftSquareBracket,
    "]" => Operator::RightSquareBracket,
    ":" => Operator::Colon,
    "/" => Operator::ForwardSlash,
    "\\" => Operator::BackSlash,
    "*" => Operator::Asterisk,
    "." => Operator::Period,
    "," => Operator::Comma,
    "_" => Operator::Underscore,
    "|" => Operator::BitOr,
    "=" => Operator::Equals,
    "<" => Operator::LessThan,
    ">" => Operator::GreaterThan,
    "~" => Operator::Tilde,
    "+" => Operator::Plus,
    "-" => Operator::Minus,
    "!" => Operator::Exclamation,
    "^" => Operator::Caret,
    "%" => Operator::Percent,
    "&" => Operator::BitAnd,
    "?" => Operator::Question,
};
