use phf::phf_map;
use phf::Map;

/// Keyword used for logic statements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Logic {
    If,
    Else,
    Match,
    Return,
}

/// Keyword used for declarative statements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Declarative {
    Record,
    Enum,
    Trait,
    Impl,
    Operator,
    Type,
    New,
    SelfType,
    Import,
    Module,
}

/// Modifier keywords.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modifier {
    Mut,
    Get,
    Set,
    Static,
    Private,
    Of,
    On,
    Auto,
    Where,
    Export,
}

/// Keyword used for primitive types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    This,
}

/// Type of keyword.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Logic(Logic),
    Declarative(Declarative),
    Primitive(Primitive),
    Modifier(Modifier),
}

static KEYWORD_MAP: Map<&'static str, Keyword> = phf_map! {
    "if" => Keyword::Logic(Logic::If),
    "else" => Keyword::Logic(Logic::Else),
    "match" => Keyword::Logic(Logic::Match),
    "ret" => Keyword::Logic(Logic::Return),

    "record" => Keyword::Declarative(Declarative::Record),
    "enum" => Keyword::Declarative(Declarative::Enum),
    "trait" => Keyword::Declarative(Declarative::Trait),
    "oper" => Keyword::Declarative(Declarative::Operator),
    "impl" => Keyword::Declarative(Declarative::Impl),
    "type" => Keyword::Declarative(Declarative::Type),
    "new" => Keyword::Declarative(Declarative::New),
    "self" => Keyword::Declarative(Declarative::SelfType),
    "import" => Keyword::Declarative(Declarative::Import),
    "module" => Keyword::Declarative(Declarative::Module),

    "mut" => Keyword::Modifier(Modifier::Mut),
    "get" => Keyword::Modifier(Modifier::Get),
    "set" => Keyword::Modifier(Modifier::Set),
    "of" => Keyword::Modifier(Modifier::Of),
    "on" => Keyword::Modifier(Modifier::On),
    "static" => Keyword::Modifier(Modifier::Static),
    "auto" => Keyword::Modifier(Modifier::Auto),
    "where" => Keyword::Modifier(Modifier::Where),
    "export" => Keyword::Modifier(Modifier::Export),

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
    "this" => Keyword::Primitive(Primitive::This),
};