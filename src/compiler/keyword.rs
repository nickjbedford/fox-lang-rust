use std::collections::HashMap;
use std::vec::Vec;
use phf::phf_map;
use phf::Map;

pub enum Logical {
    If,
    Else,
    Match
}

pub enum Declarative {
    Data,
    Impl,
    Operator
}

pub enum Primitive {
    Char,
    Short,
    Int,
    Long,
    Byte,
    Word,
    UInt,
    ULong,
    Float,
    Double,
    String
}

pub enum Keyword {
    Logical(Logical),
    Declarative(Declarative),
    Primitive(Primitive)
}

static KEYWORD_MAP: Map<&'static str, Keyword> = phf_map! {
    "if" => Keyword::Logical(Logical::If),
    "else" => Keyword::Logical(Logical::Else),
    "match" => Keyword::Logical(Logical::Match),

    "data" => Keyword::Declarative(Declarative::Data),
    "impl" => Keyword::Declarative(Declarative::Impl),
    "operator" => Keyword::Declarative(Declarative::Operator),

    "char" => Keyword::Primitive(Primitive::Char),
    "short" => Keyword::Primitive(Primitive::Short),
    "int" => Keyword::Primitive(Primitive::Int),
    "long" => Keyword::Primitive(Primitive::Long),

    "byte" => Keyword::Primitive(Primitive::Byte),
    "word" => Keyword::Primitive(Primitive::Word),
    "uint" => Keyword::Primitive(Primitive::UInt),
    "ulong" => Keyword::Primitive(Primitive::ULong),

    "float" => Keyword::Primitive(Primitive::Float),
    "double" => Keyword::Primitive(Primitive::Double),
    
    "string" => Keyword::Primitive(Primitive::String),
};