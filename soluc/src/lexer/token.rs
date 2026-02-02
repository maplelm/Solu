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
    GtGt,  // >>
    Lt,    // <
    LtLt,  // <<
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
                Operator::GtGt => write!(f, ">>"),
                Operator::Lt => write!(f, "<"),
                Operator::LtLt => write!(f, "<<"),
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

// Dummy functions for pattern matching
impl Token {
    pub const IDENTIFER: Token = Token::Identifier(String::new());
    pub const INT: Token = Self::Num(NumLiteral::Int(0));
    pub const FLOAT: Token = Self::Num(NumLiteral::Float(0.0));
    pub const STRING: Token = Self::Str(String::new());
}

// Keyword Easy Access
impl Token {
    pub const KEY_IF: Token = Self::Keyword(Keyword::If);
    pub const KEY_THEN: Token = Self::Keyword(Keyword::Then);
    pub const KEY_ELSE: Token = Self::Keyword(Keyword::Else);
    pub const KEY_ELIF: Token = Self::Keyword(Keyword::Elif);
    pub const KEY_WHILE: Token = Self::Keyword(Keyword::While);
    pub const KEY_FOR: Token = Self::Keyword(Keyword::For);
    pub const KEY_IN: Token = Self::Keyword(Keyword::In);
    pub const KEY_DO: Token = Self::Keyword(Keyword::Do);
    pub const KEY_WITH: Token = Self::Keyword(Keyword::With);
    pub const KEY_IS: Token = Self::Keyword(Keyword::Is);
    pub const KEY_END: Token = Self::Keyword(Keyword::End);
    pub const KEY_RETURN: Token = Self::Keyword(Keyword::Return);
    pub const KEY_BREAK: Token = Self::Keyword(Keyword::Break);
    pub const KEY_CONTINUE: Token = Self::Keyword(Keyword::Continue);
    pub const KEY_SWITCH: Token = Self::Keyword(Keyword::Switch);
    pub const KEY_MUT: Token = Self::Keyword(Keyword::Mut);
    pub const KEY_STRUCT: Token = Self::Keyword(Keyword::Struct);
    pub const KEY_ENUM: Token = Self::Keyword(Keyword::Enum);
    pub const KEY_CONST: Token = Self::Keyword(Keyword::Const);
    pub const KEY_NAMESPACE: Token = Self::Keyword(Keyword::Namespace);
    pub const KEY_TYPE: Token = Self::Keyword(Keyword::Type);
    pub const KEY_ARENA: Token = Self::Keyword(Keyword::Arena);
    pub const KEY_DEFER: Token = Self::Keyword(Keyword::Defer);
    pub const KEY_NEW: Token = Self::Keyword(Keyword::New);
    pub const KEY_TRUE: Token = Self::Keyword(Keyword::True);
    pub const KEY_FALSE: Token = Self::Keyword(Keyword::False);
    pub const KEY_NIL: Token = Self::Keyword(Keyword::Nil);
    pub const KEY_I8: Token = Self::Keyword(Keyword::TypeI8);
    pub const KEY_I16: Token = Self::Keyword(Keyword::TypeI16);
    pub const KEY_I32: Token = Self::Keyword(Keyword::TypeI32);
    pub const KEY_I64: Token = Self::Keyword(Keyword::TypeI64);
    pub const KEY_U8: Token = Self::Keyword(Keyword::TypeU8);
    pub const KEY_U16: Token = Self::Keyword(Keyword::TypeU16);
    pub const KEY_U32: Token = Self::Keyword(Keyword::TypeU32);
    pub const KEY_U64: Token = Self::Keyword(Keyword::TypeU64);
    pub const KEY_F32: Token = Self::Keyword(Keyword::TypeF32);
    pub const KEY_F64: Token = Self::Keyword(Keyword::TypeF64);
    pub const KEY_CHAR: Token = Self::Keyword(Keyword::TypeChar);
    pub const KEY_BOOL: Token = Self::Keyword(Keyword::TypeBool);

    // Operator
    pub const OP_NOT: Token = Self::Operator(Operator::Not);
    pub const OP_TERINARY: Token = Self::Operator(Operator::Terinary);
    pub const OP_EQEQ: Token = Self::Operator(Operator::EqEq);
    pub const OP_NOTEQ: Token = Self::Operator(Operator::NotEq);
    pub const OP_GT: Token = Self::Operator(Operator::Gt);
    pub const OP_GTGT: Token = Self::Operator(Operator::GtGt);
    pub const OP_LT: Token = Self::Operator(Operator::Lt);
    pub const OP_LTLT: Token = Self::Operator(Operator::LtLt);
    pub const OP_GTEQ: Token = Self::Operator(Operator::GtEq);
    pub const OP_LTEQ: Token = Self::Operator(Operator::LtEq);
    pub const OP_EQ: Token = Self::Operator(Operator::Eq);
    pub const OP_LAND: Token = Self::Operator(Operator::LAnd);
    pub const OP_LOR: Token = Self::Operator(Operator::LOr);
    pub const OP_PLUSEQ: Token = Self::Operator(Operator::PlusEq);
    pub const OP_SUBEQ: Token = Self::Operator(Operator::SubEq);
    pub const OP_STAREQ: Token = Self::Operator(Operator::StarEq);
    pub const OP_DIVEQ: Token = Self::Operator(Operator::DivEq);
    pub const OP_MODEQ: Token = Self::Operator(Operator::ModEq);
    pub const OP_PLUS: Token = Self::Operator(Operator::Plus);
    pub const OP_STAR: Token = Self::Operator(Operator::Star);
    pub const OP_DIV: Token = Self::Operator(Operator::Div);
    pub const OP_MINUS: Token = Self::Operator(Operator::Minus);
    pub const OP_MOD: Token = Self::Operator(Operator::Mod);
    pub const OP_AMP: Token = Self::Operator(Operator::Amp);
    pub const OP_AMPEQ: Token = Self::Operator(Operator::AmpEq);
    pub const OP_PIPE: Token = Self::Operator(Operator::Pipe);
    pub const OP_PIPEEQ: Token = Self::Operator(Operator::PipeEq);
    pub const OP_CARET: Token = Self::Operator(Operator::Caret);
    pub const OP_CARETEQ: Token = Self::Operator(Operator::CaretEq);
    pub const OP_TILDE: Token = Self::Operator(Operator::Tilde);
    pub const OP_TILDEEQ: Token = Self::Operator(Operator::TildeEq);
    pub const OP_SHIFTL: Token = Self::Operator(Operator::ShiftL);
    pub const OP_SHIFTR: Token = Self::Operator(Operator::ShiftR);
    pub const OP_ARROW: Token = Self::Operator(Operator::Arrow);
    pub const OP_ARROWREV: Token = Self::Operator(Operator::ArrowRev);
    pub const OP_FATARROW: Token = Self::Operator(Operator::FatArrow);
    pub const OP_RANGE: Token = Self::Operator(Operator::Range);
    pub const OP_MEMBERACCESSOR: Token = Self::Operator(Operator::MemberAccessor);

    // Delimeter
    pub const LPAREN: Token = Self::Delim(Delimeter::Lparen);
    pub const RPAREN: Token = Self::Delim(Delimeter::Rparen);
    pub const LBRACKET: Token = Self::Delim(Delimeter::Lbracket);
    pub const RBRACKET: Token = Self::Delim(Delimeter::Rbracket);
    pub const LBRACE: Token = Self::Delim(Delimeter::Lbrace);
    pub const RBRACE: Token = Self::Delim(Delimeter::Rbrace);
    pub const COLON: Token = Self::Delim(Delimeter::Colon);
    pub const COMMA: Token = Self::Delim(Delimeter::Comma);
    pub const SEMICOLON: Token = Self::Delim(Delimeter::SemiColon);
    pub const TERM: Token = Self::Delim(Delimeter::Term);
    // Special
    pub const EOF: Token = Self::Special(Special::Eof);
    pub fn invalid(s: impl Into<String>) -> Self {
        Token::Special(Special::Invalid(s.into()))
    }
}

impl Token {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Self::Identifier(s) => Some(s.clone()),
            Self::Char(_) => None,
            Self::Str(s) => Some(s.clone()),
            Token::Num(_) => None,
            Self::Keyword(_) => None,
            Self::Operator(_) => None,
            Self::Delim(_) => None,
            Self::Special(m) => match m {
                Special::Invalid(s) => Some(s.clone()),
                _ => None,
            },
        }
    }
}
