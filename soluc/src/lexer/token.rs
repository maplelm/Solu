use crate::lexer::{LexerError, Span};
use std::fmt;

#[derive(Debug, Clone)]
pub struct LexerObject {
    pub token: Token,
    pub span: Span,
}

impl LexerObject {
    pub fn from_vectors(tokens: &[Token], spans: &[Span]) -> Result<Vec<Self>, String> {
        if tokens.len() != spans.len() {
            return Err(format!(
                "LexerObject List Creation Failed, Must have equal tokens and spans | {} Tokens , {} Spans",
                tokens.len(),
                spans.len()
            ));
        }

        let mut objects_list = Vec::with_capacity(tokens.len());
        for (i, tok) in tokens.iter().enumerate() {
            objects_list.push(Self {
                token: tok.clone(),
                span: spans[i],
            });
        }
        Ok(objects_list)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    Amp,   // &
    AmpEq, // &=
    Arena,
    Arrow,    // ->
    ArrowRev, // <-
    Break,
    Caret,   // ^
    CaretEq, // ^=
    Case,
    Char(char),
    Colon, // :
    Comma, // ,
    Const,
    Continue,
    Default,
    Defer,
    Div,   // /
    DivEq, // /=
    Do,
    Elif,
    Else,
    End,
    Enum,
    Eof,
    Eq,   // =
    EqEq, // ==
    False,
    FatArrow, // =>
    Float(f64),
    For,
    Gt,   // >
    GtEq, // >=
    GtGt, // >>
    Identifier(String),
    If,
    In,
    Int(u64),
    Invalid(String),
    Is,
    // Keywords
    LAnd,           // Logic And
    Lbrace,         // {
    Lbracket,       // [
    LOr,            // Logic Or
    Lparen,         // (
    Lt,             // <
    LtEq,           // <=
    LtLt,           // <<
    MemberAccessor, // .
    Minus,          // -
    Mod,            // %
    ModEq,          // %=
    Mut,
    Namespace,
    New,
    Nil,
    Not,
    NotEq, // !=
    // Number Literals
    // Operators
    Pipe,     // |
    PipeEq,   // |=
    Plus,     // +
    PlusEq,   // +=
    Range,    // ..
    Rbrace,   // }
    Rbracket, // ]
    Return,
    Rparen,    // )
    SemiColon, // ;
    ShiftL,    // <<
    ShiftR,    // >>
    Star,      // *
    StarEq,    // *=
    Str(String),
    Struct,
    SubEq, // -=
    Switch,
    Terinary, // a ? b : c
    Term,     // \n
    Then,
    This,
    Tilde,   // ~
    TildeEq, // ~=
    True,
    Type,
    TypeBool,
    TypeChar,
    TypeF32,
    TypeF64,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeI8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeU8,
    While,
    With,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::AmpEq => write!(f, "&="),
            Token::Amp => write!(f, "&"),
            Token::Arena => write!(f, "ARENA"),
            Token::ArrowRev => write!(f, "<-"),
            Token::Arrow => write!(f, "->"),
            Token::Break => write!(f, "BREAK"),
            Token::CaretEq => write!(f, "^="),
            Token::Caret => write!(f, "^"),
            Token::Case => write!(f, "CASE"),
            Token::Char(c) => write!(f, "CHAR({})", c),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Const => write!(f, "CONST"),
            Token::Continue => write!(f, "CONTINUE"),
            Token::Default => write!(f, "DEFAULT"),
            Token::Defer => write!(f, "DEFER"),
            Token::DivEq => write!(f, "/="),
            Token::Div => write!(f, "/"),
            Token::Do => write!(f, "DO"),
            Token::Elif => write!(f, "ELIF"),
            Token::Else => write!(f, "ELSE"),
            Token::End => write!(f, "END"),
            Token::Enum => write!(f, "ENUM"),
            Token::Eof => write!(f, "EOF"),
            Token::EqEq => write!(f, "=="),
            Token::Eq => write!(f, "="),
            Token::False => write!(f, "FALSE"),
            Token::FatArrow => write!(f, "=>"),
            Token::Float(fl) => write!(f, "FLOAT({})", fl),
            Token::For => write!(f, "FOR"),
            Token::GtEq => write!(f, ">="),
            Token::GtGt => write!(f, ">>"),
            Token::Gt => write!(f, ">"),
            Token::Identifier(s) => write!(f, "IDENT({})", s),
            Token::If => write!(f, "IF"),
            Token::Int(i) => write!(f, "INT({})", i),
            Token::Invalid(s) => write!(f, "INVALID({})", s),
            Token::In => write!(f, "IN"),
            Token::Is => write!(f, "IS"),
            Token::LAnd => write!(f, "&&"),
            Token::Lbrace => write!(f, "{{"),
            Token::Lbracket => write!(f, "["),
            Token::LOr => write!(f, "||"),
            Token::Lparen => write!(f, "("),
            Token::LtEq => write!(f, "<="),
            Token::LtLt => write!(f, "<<"),
            Token::Lt => write!(f, "<"),
            Token::MemberAccessor => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::ModEq => write!(f, "%="),
            Token::Mod => write!(f, "%"),
            Token::Mut => write!(f, "MUT"),
            Token::Namespace => write!(f, "NAMESPACE"),
            Token::New => write!(f, "NEW"),
            Token::Nil => write!(f, "NIL"),
            Token::NotEq => write!(f, "!="),
            Token::Not => write!(f, "!"),
            Token::PipeEq => write!(f, "|="),
            Token::Pipe => write!(f, "|"),
            Token::PlusEq => write!(f, "+="),
            Token::Plus => write!(f, "+"),
            Token::Range => write!(f, ".."),
            Token::Rbrace => write!(f, "}}"),
            Token::Rbracket => write!(f, "]"),
            Token::Return => write!(f, "RETURN"),
            Token::Rparen => write!(f, ")"),
            Token::SemiColon => write!(f, ";"),
            Token::ShiftL => write!(f, "<<"),
            Token::ShiftR => write!(f, ">>"),
            Token::StarEq => write!(f, "*="),
            Token::Star => write!(f, "*"),
            Token::Str(s) => write!(f, "STR({})", s),
            Token::Struct => write!(f, "STRUCT"),
            Token::SubEq => write!(f, "-="),
            Token::Switch => write!(f, "SWITCH"),
            Token::Terinary => write!(f, "?"),
            Token::Term => write!(f, "TERM"),
            Token::Then => write!(f, "THEN"),
            Token::This => write!(f, "THIS"),
            Token::TildeEq => write!(f, "~="),
            Token::Tilde => write!(f, "~"),
            Token::True => write!(f, "TRUE"),
            Token::TypeBool => write!(f, "BOOL"),
            Token::TypeChar => write!(f, "CHAR"),
            Token::TypeF32 => write!(f, "F32"),
            Token::TypeF64 => write!(f, "F64"),
            Token::TypeI16 => write!(f, "I16"),
            Token::TypeI32 => write!(f, "I32"),
            Token::TypeI64 => write!(f, "I64"),
            Token::TypeI8 => write!(f, "I8"),
            Token::TypeU16 => write!(f, "U16"),
            Token::TypeU32 => write!(f, "U32"),
            Token::TypeU64 => write!(f, "U64"),
            Token::TypeU8 => write!(f, "U8"),
            Token::Type => write!(f, "TYPE"),
            Token::While => write!(f, "WHILE"),
            Token::With => write!(f, "WITH"),
        }
    }
}

// Dummy functions for pattern matching
impl Token {
    pub const IDENTIFER: Token = Token::Identifier(String::new());
    pub const INT: Token = Self::Int(0);
    pub const FLOAT: Token = Self::Float(0.0);
    pub const STRING: Token = Self::Str(String::new());
}

// Keyword Easy Access
impl Token {
    pub const KEY_IF: Token = Self::If;
    pub const KEY_THEN: Token = Self::Then;
    pub const KEY_ELSE: Token = Self::Else;
    pub const KEY_ELIF: Token = Self::Elif;
    pub const KEY_WHILE: Token = Self::While;
    pub const KEY_FOR: Token = Self::For;
    pub const KEY_IN: Token = Self::In;
    pub const KEY_DO: Token = Self::Do;
    pub const KEY_WITH: Token = Self::With;
    pub const KEY_IS: Token = Self::Is;
    pub const KEY_END: Token = Self::End;
    pub const KEY_RETURN: Token = Self::Return;
    pub const KEY_BREAK: Token = Self::Break;
    pub const KEY_CONTINUE: Token = Self::Continue;
    pub const KEY_SWITCH: Token = Self::Switch;
    pub const KEY_CASE: Token = Self::Case;
    pub const KEY_DEFAULT: Token = Self::Default;
    pub const KEY_MUT: Token = Self::Mut;
    pub const KEY_STRUCT: Token = Self::Struct;
    pub const KEY_ENUM: Token = Self::Enum;
    pub const KEY_CONST: Token = Self::Const;
    pub const KEY_NAMESPACE: Token = Self::Namespace;
    pub const KEY_TYPE: Token = Self::Type;
    pub const KEY_ARENA: Token = Self::Arena;
    pub const KEY_DEFER: Token = Self::Defer;
    pub const KEY_NEW: Token = Self::New;
    pub const KEY_TRUE: Token = Self::True;
    pub const KEY_FALSE: Token = Self::False;
    pub const KEY_NIL: Token = Self::Nil;
    pub const KEY_I8: Token = Self::TypeI8;
    pub const KEY_I16: Token = Self::TypeI16;
    pub const KEY_I32: Token = Self::TypeI32;
    pub const KEY_I64: Token = Self::TypeI64;
    pub const KEY_U8: Token = Self::TypeU8;
    pub const KEY_U16: Token = Self::TypeU16;
    pub const KEY_U32: Token = Self::TypeU32;
    pub const KEY_U64: Token = Self::TypeU64;
    pub const KEY_F32: Token = Self::TypeF32;
    pub const KEY_F64: Token = Self::TypeF64;
    pub const KEY_CHAR: Token = Self::TypeChar;
    pub const KEY_BOOL: Token = Self::TypeBool;

    // Operator
    pub const OP_NOT: Token = Self::Not;
    pub const OP_TERINARY: Token = Self::Terinary;
    pub const OP_EQEQ: Token = Self::EqEq;
    pub const OP_NOTEQ: Token = Self::NotEq;
    pub const OP_GT: Token = Self::Gt;
    pub const OP_GTGT: Token = Self::GtGt;
    pub const OP_LT: Token = Self::Lt;
    pub const OP_LTLT: Token = Self::LtLt;
    pub const OP_GTEQ: Token = Self::GtEq;
    pub const OP_LTEQ: Token = Self::LtEq;
    pub const OP_EQ: Token = Self::Eq;
    pub const OP_LAND: Token = Self::LAnd;
    pub const OP_LOR: Token = Self::LOr;
    pub const OP_PLUSEQ: Token = Self::PlusEq;
    pub const OP_SUBEQ: Token = Self::SubEq;
    pub const OP_STAREQ: Token = Self::StarEq;
    pub const OP_DIVEQ: Token = Self::DivEq;
    pub const OP_MODEQ: Token = Self::ModEq;
    pub const OP_PLUS: Token = Self::Plus;
    pub const OP_STAR: Token = Self::Star;
    pub const OP_DIV: Token = Self::Div;
    pub const OP_MINUS: Token = Self::Minus;
    pub const OP_MOD: Token = Self::Mod;
    pub const OP_AMP: Token = Self::Amp;
    pub const OP_AMPEQ: Token = Self::AmpEq;
    pub const OP_PIPE: Token = Self::Pipe;
    pub const OP_PIPEEQ: Token = Self::PipeEq;
    pub const OP_CARET: Token = Self::Caret;
    pub const OP_CARETEQ: Token = Self::CaretEq;
    pub const OP_TILDE: Token = Self::Tilde;
    pub const OP_TILDEEQ: Token = Self::TildeEq;
    pub const OP_SHIFTL: Token = Self::ShiftL;
    pub const OP_SHIFTR: Token = Self::ShiftR;
    pub const OP_ARROW: Token = Self::Arrow;
    pub const OP_ARROWREV: Token = Self::ArrowRev;
    pub const OP_FATARROW: Token = Self::FatArrow;
    pub const OP_RANGE: Token = Self::Range;
    pub const OP_MEMBERACCESSOR: Token = Self::MemberAccessor;

    // Delimeter
    pub const LPAREN: Token = Self::Lparen;
    pub const RPAREN: Token = Self::Rparen;
    pub const LBRACKET: Token = Self::Lbracket;
    pub const RBRACKET: Token = Self::Rbracket;
    pub const LBRACE: Token = Self::Lbrace;
    pub const RBRACE: Token = Self::Rbrace;
    pub const COLON: Token = Self::Colon;
    pub const COMMA: Token = Self::Comma;
    pub const SEMICOLON: Token = Self::SemiColon;
    pub const TERM: Token = Self::Term;
    // Special
    pub const THIS: Token = Self::This;
    pub const EOF: Token = Self::Eof;
    pub fn invalid(s: impl Into<String>) -> Self {
        Self::Invalid(s.into())
    }
}

impl Token {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Self::Identifier(s) => Some(s.clone()),
            Self::Str(s) => Some(s.clone()),
            Self::Invalid(s) => Some(s.clone()),
            _ => None,
        }
    }
}
