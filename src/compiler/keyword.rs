use phf::phf_map;
use phf::Map;

pub enum Logical {
    If,
    Else,
    Match,
}

pub enum Declarative {
    Record,
    Impl,
    Operator
}

pub enum Primitive {
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float,
    Double,
    Decimal,
    String,
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

    "record" => Keyword::Declarative(Declarative::Record),
    "oper" => Keyword::Declarative(Declarative::Operator),

    "char" => Keyword::Primitive(Primitive::Int8),
    "short" => Keyword::Primitive(Primitive::Int16),
    "int" => Keyword::Primitive(Primitive::Int32),
    "long" => Keyword::Primitive(Primitive::Int64),

    "u8" => Keyword::Primitive(Primitive::UInt8),
    "u16" => Keyword::Primitive(Primitive::UInt16),
    "u32" => Keyword::Primitive(Primitive::UInt32),
    "u64" => Keyword::Primitive(Primitive::UInt64),

    "f32" => Keyword::Primitive(Primitive::Float),
    "f64" => Keyword::Primitive(Primitive::Double),
    "decimal" => Keyword::Primitive(Primitive::Decimal),
    
    "string" => Keyword::Primitive(Primitive::String),
};