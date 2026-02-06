#![allow(unused)]
use super::core::Parser;
use crate::lexer::{self, Span};
use std::fmt;

pub type Program = Vec<lexer::Token>;
pub type Ast = Vec<Namespace>;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub nodes: Vec<Decl>,
}

impl fmt::Display for Namespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Namespace: {}\n", self.name);
        for n in self.nodes.iter() {
            write!(f, "\t{:?}", n);
        }

        write!(f, "\n")
    }
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
        name: String,
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
    VarDeclare {
        mutable: bool,
        name: String,
        kind: Type,
        init: Option<Expr>,
    },
    Assign {
        left: Expr,
        op: AssignOp,
        right: Expr,
    },
    Expr(Expr),
    Return(Option<Expr>),
    Break,
    Continue,
    If {
        cond: Expr,
        elif_branches: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
        body: Vec<Stmt>,
    },
    Elif,
    Else,
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    For {
        ident: String,
        iter: Iterator,
        body: Vec<Stmt>,
    },
    Switch {
        cond: Expr,
        cases: Vec<SwitchCase>,
    },
}

#[derive(Debug, Clone)]
pub enum Iterator {
    Object(Expr),
    Range { min: Expr, max: Expr },
}

#[derive(Debug, Clone)]
pub enum SwitchCase {
    Case { pattern: Expr, body: Vec<Stmt> },
    Default(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum AssignOp {
    Eq,     // =
    PlusEq, // +=
    SubEq,  // -=
    MulEq,  // *=
    DivEq,  // /=
    ModEq,  // %=
    AndEq,  // &=
    OrEq,   // |=
    XorEq,  // ^=
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub parser: Parser,
    pub msg: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: ({:?} {:?} {:?}) ({}) {}",
            self.parser.span_object(),
            self.parser.previous(),
            self.parser.peek(),
            self.parser.next(),
            self.parser.pos,
            self.msg
        )
    }
}

impl ParserError {
    pub fn new(parser: &Parser, msg: impl Into<String>) -> Self {
        Self {
            parser: parser.clone(),
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
    Array { kind: TypeBase, size: Box<Expr> },
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
    Char(char),
    String(String),
    Nil,
    False,
    This,
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
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
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
            _ => None,
        }
    }
}
