use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Identifier(String),
    Num(NumLiteral),
    Char(char),
    Str(String),
    Keyword(Keyword),
    Operator(Operator),
    Delim(Delimeter),
    Special(Special),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Special {
    Invalid(String),
    Eof,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Delimeter {
    Lparen,    // (
    Rparen,    // )
    Lbracket,  // [
    Rbracket,  // ]
    Lbrace,    // {
    Rbrace,    // }
    Colon,     // :
    Comma,     // ,
    SemiColon, // ;
    Term,      // \n
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum NumLiteral {
    Int(u64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Keyword {
    // Control Flow
    If,
    Then,
    Else,
    Elif,
    While,
    For,
    In,
    Do,
    With,
    Is,
    End,
    Return,
    Break,
    Continue,
    Switch,
    // Declarations
    Mut,
    Struct,
    Enum,
    Const,
    Namespace,
    Type,
    // Memory
    Arena,
    Defer,
    New,
    // Literals
    True,
    False,
    Nil,
    // Built in Types
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeF32,
    TypeF64,
    TypeChar,
    TypeBool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operator {
    //Logic
    Not,
    Terinary, // a ? b : c
    // Comparison
    EqEq,  // ==
    NotEq, // !=
    Gt,    // >
    Lt,    // <
    GtEq,  // >=
    LtEq,  // <=
    LAnd,  // Logic And
    LOr,   // Logic Or
    // Assignment
    Eq,     // =
    PlusEq, // +=
    SubEq,  // -=
    StarEq, // *=
    DivEq,  // /=
    ModEq,  // %=
    // Arithmatic
    Plus,  // +
    Star,  // *
    Div,   // /
    Minus, // -
    Mod,   // %
    // Bitwise
    Amp,     // &
    AmpEq,   // &=
    Pipe,    // |
    PipeEq,  // |=
    Caret,   // ^
    CaretEq, // ^=
    Tilde,   // ~
    TildeEq, // ~=
    ShiftL,  // <<
    ShiftR,  // >>
    //Memory
    Arrow,          // ->
    ArrowRev,       // <-
    FatArrow,       // =>
    Range,          // ..
    MemberAccessor, // .
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(f, "IDENT({})", s),
            Token::Char(c) => write!(f, "CHAR({})", c),
            Token::Str(s) => write!(f, "STR({})", s),
            Token::Num(n) => match n {
                NumLiteral::Int(i) => write!(f, "INT({})", i),
                NumLiteral::Float(fl) => write!(f, "FLOAT({})", fl),
            },
            Token::Keyword(w) => match w {
                Keyword::If => write!(f, "IF"),
                Keyword::Then => write!(f, "THEN"),
                Keyword::Else => write!(f, "ELSE"),
                Keyword::Elif => write!(f, "ELIF"),
                Keyword::While => write!(f, "WHILE"),
                Keyword::For => write!(f, "FOR"),
                Keyword::In => write!(f, "IN"),
                Keyword::Do => write!(f, "DO"),
                Keyword::With => write!(f, "WITH"),
                Keyword::Is => write!(f, "IS"),
                Keyword::End => write!(f, "END"),
                Keyword::Return => write!(f, "RETURN"),
                Keyword::Break => write!(f, "BREAK"),
                Keyword::Continue => write!(f, "CONTINUE"),
                Keyword::Switch => write!(f, "SWITCH"),
                Keyword::Mut => write!(f, "MUT"),
                Keyword::Struct => write!(f, "STRUCT"),
                Keyword::Enum => write!(f, "ENUM"),
                Keyword::Const => write!(f, "CONST"),
                Keyword::Namespace => write!(f, "NAMESPACE"),
                Keyword::Type => write!(f, "TYPE"),
                Keyword::Arena => write!(f, "ARENA"),
                Keyword::Defer => write!(f, "DEFER"),
                Keyword::New => write!(f, "NEW"),
                Keyword::True => write!(f, "TRUE"),
                Keyword::False => write!(f, "FALSE"),
                Keyword::Nil => write!(f, "NIL"),
                Keyword::TypeI8 => write!(f, "I8"),
                Keyword::TypeI16 => write!(f, "I16"),
                Keyword::TypeI32 => write!(f, "I32"),
                Keyword::TypeI64 => write!(f, "I64"),
                Keyword::TypeU8 => write!(f, "U8"),
                Keyword::TypeU16 => write!(f, "U16"),
                Keyword::TypeU32 => write!(f, "U32"),
                Keyword::TypeU64 => write!(f, "U64"),
                Keyword::TypeF32 => write!(f, "F32"),
                Keyword::TypeF64 => write!(f, "F64"),
                Keyword::TypeChar => write!(f, "CHAR"),
                Keyword::TypeBool => write!(f, "BOOL"),
            },
            Token::Operator(op) => match op {
                Operator::Not => write!(f, "!"),
                Operator::Terinary => write!(f, "?"),
                Operator::EqEq => write!(f, "=="),
                Operator::NotEq => write!(f, "!="),
                Operator::Gt => write!(f, ">"),
                Operator::Lt => write!(f, "<"),
                Operator::GtEq => write!(f, ">="),
                Operator::LtEq => write!(f, "<="),
                Operator::Eq => write!(f, "="),
                Operator::LAnd => write!(f, "&&"),
                Operator::LOr => write!(f, "||"),
                Operator::PlusEq => write!(f, "+="),
                Operator::SubEq => write!(f, "-="),
                Operator::StarEq => write!(f, "*="),
                Operator::DivEq => write!(f, "/="),
                Operator::ModEq => write!(f, "%="),
                Operator::Plus => write!(f, "+"),
                Operator::Star => write!(f, "*"),
                Operator::Div => write!(f, "/"),
                Operator::Minus => write!(f, "-"),
                Operator::Mod => write!(f, "%"),
                Operator::Amp => write!(f, "&"),
                Operator::AmpEq => write!(f, "&="),
                Operator::Pipe => write!(f, "|"),
                Operator::PipeEq => write!(f, "|="),
                Operator::Caret => write!(f, "^"),
                Operator::CaretEq => write!(f, "^="),
                Operator::Tilde => write!(f, "~"),
                Operator::TildeEq => write!(f, "~="),
                Operator::ShiftL => write!(f, "<<"),
                Operator::ShiftR => write!(f, ">>"),
                Operator::Arrow => write!(f, "->"),
                Operator::ArrowRev => write!(f, "<-"),
                Operator::FatArrow => write!(f, "=>"),
                Operator::Range => write!(f, ".."),
                Operator::MemberAccessor => write!(f, "."),
            },
            Token::Delim(del) => match del {
                Delimeter::Lparen => write!(f, "("),
                Delimeter::Rparen => write!(f, ")"),
                Delimeter::Lbracket => write!(f, "["),
                Delimeter::Rbracket => write!(f, "]"),
                Delimeter::Lbrace => write!(f, "{{"),
                Delimeter::Rbrace => write!(f, "}}"),
                Delimeter::Colon => write!(f, ":"),
                Delimeter::Comma => write!(f, ","),
                Delimeter::SemiColon => write!(f, ";"),
                Delimeter::Term => write!(f, "TERM"),
            },
            Token::Special(s) => match s {
                Special::Invalid(s) => write!(f, "INVALID({})", s),
                Special::Eof => write!(f, "EOF"),
            },
        }
    }
}
