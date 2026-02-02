#![allow(unused)]
use super::core::Parser;
use crate::lexer;
use std::fmt;

pub type Program = Vec<lexer::Token>;
pub type Ast = Vec<Namespace>;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub nodes: Vec<Decl>,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Function {
        name: String,
        params: Vec<Param>,
        ret_type: Option<Type>,
        body: Vec<Stmt>,
    },
    Member {
        name: String,
        parent: String,
        params: Vec<Param>,
        ret_type: Option<Type>,
        body: Vec<Stmt>,
    },
    Struct {
        name: String
        fields: Vec<StructFieldDecl>,
    },
    Enum {
        name: String,
        variants: Vec<String>,
    },
    Namespace(Namespace),
    Const {
        name: String,
        kind: Type,
        value: Expr,
    },
}

#[derive(Debug, Clone)]
pub struct StructFieldDecl {
    pub name: String,
    pub kind: Type,
}

#[derive(Debug, Clone)]
pub enum Stmt {
}

#[derive(Debug, Clone)]
pub struct ParserError {
    span: lexer::Span,
    msg: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse program {}, {}", self.span, self.msg)
    }
}

impl ParserError {
    pub fn new(par: &Parser, msg: impl Into<String>) -> Self {
        Self {
            span: lexer::Span {
                start: par.pos,
                end: par.pos,
                line: par.pos,
                col: par.pos,
            },
            msg: msg.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ArraySize {
    Usize(usize),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Array { kind: TypeBase, size: ArraySize },
    Base(TypeBase),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TypeBase {
    Ident(String),
    Prim(TypePrim),
    Ptr(Box<Type>),
    Ref(Box<Type>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TypePrim {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Char,
    Bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Param {
    pub name: String,
    pub kind: Type,
}

/*
* a << b || c && d ^ z
* a || b || c && d && e || f
*
* a >> b || c >> d
* a >> b >> c
*/
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    // Ternary Expressions
    Ternary {
        a: Box<Expr>,
        b: Box<Expr>,
        c: Box<Expr>,
    },
    // Binary Expressions
    LogicOr {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LogicAnd {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    BitwiseOr {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    BitwiseShiftLeft {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    BitwiseShiftRight {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Cmp {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotCmp {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Gt {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Lt {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LtEq {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GtEq {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Sub {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Mul {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Div {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Mod {
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Unary Expressions
    Neg {
        expr: Box<Expr>,
    },
    Not {
        expr: Box<Expr>,
    },
    Tilde {
        expr: Box<Expr>,
    },
    Cast {
        kind: Type,
        expr: Box<Expr>,
    },
    Ident(String),
    StructLiteral {
        name: String,
        fields: Vec<StructFieldInit>,
    },
    String(String),
    Nil,
    False,
    True,
    Int(u64),
    Float(f64),
    FuncCall {
        name: String,
        params: Vec<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Member {
        object: Box<Expr>,
        name: String,
    },
    Index{
        array: Box<Expr>
        index: Box<Expr>
    },
    Deref(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StructFieldInit {
    pub identifier: String,
    pub value: Expr,
}

impl Expr {
    pub fn to_string(&self) -> Option<String> {
        match self {
            Self::Ident(s) => Some(s.clone()),
            Self::String(s) => Some(s.clone()),
            _ => None
        }
    }
}

