use std::collections::HashMap;
use std::vec::Vec;
use phf::phf_map;
use phf::Map;

pub enum Operator
{
    LeftBracket,
    RightBracket,
    LeftSquareBracket,
    RightSquareBracket,
    ColonAssignment,
    Colon,
    ForwardSlash,
    BackSlash,
    Hyphen,
    Asterisk,
    Period,
}

static OPERATOR_MAP: Map<&'static str, Operator> = phf_map!
{
    "(" => Operator::LeftBracket,
    ")" => Operator::RightBracket,
};
