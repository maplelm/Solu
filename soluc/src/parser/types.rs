#![allow(unused)]
use super::core::Parser;
use crate::lexer::{self, Span};
use std::fmt::{self, Write};
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

//////////////////
// Declarations //
//////////////////

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StructFieldDecl {
    pub name: String,
    pub kind: Type,
}

////////////////
// Statements //
////////////////

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

////////////////
// Error Type //
////////////////

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

////////////////
// Data Types //
////////////////

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ArraySize {
    Usize(usize),
    Identifier(QualifiedName),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Type {
    Array { kind: TypeBase, size: Box<Expr> },
    Base(TypeBase),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base(t) => write!(f, "{}", t),
            Self::Array { kind, size } => write!(f, "{}[{}]", kind, size),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TypeBase {
    Ident(QualifiedName),
    Prim(TypePrim),
    Enum,
    Ptr(Box<Type>),
    Ref(Box<Type>),
}

impl fmt::Display for TypeBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(s) => write!(f, "{}", s),
            Self::Prim(p) => write!(f, "{}", p),
            Self::Enum => write!(f, "Enum"),
            Self::Ptr(t) => write!(f, "*{}", t),
            Self::Ref(t) => write!(f, "&{}", t),
        }
    }
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

impl fmt::Display for TypePrim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8 => write!(f, "I8"),
            Self::I16 => write!(f, "I16"),
            Self::I32 => write!(f, "I32"),
            Self::I64 => write!(f, "I64"),
            Self::U8 => write!(f, "U8"),
            Self::U16 => write!(f, "U16"),
            Self::U32 => write!(f, "U32"),
            Self::U64 => write!(f, "U64"),
            Self::F32 => write!(f, "F32"),
            Self::F64 => write!(f, "F64"),
            Self::Char => write!(f, "Char"),
            Self::Bool => write!(f, "Bool"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Param {
    pub name: String,
    pub kind: Type,
}

////////////////
// Expresions //
////////////////

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
    Ident(QualifiedName),
    StructLiteral {
        name: QualifiedName,
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
        name: QualifiedName,
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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ternary { a, b, c } => write!(f, "{} ? {} : {}", a, b, c),
            Self::LogicOr { left, right } => write!(f, "{} || {}", left, right),
            Self::LogicAnd { left, right } => write!(f, "{} && {}", left, right),
            Self::BitwiseOr { left, right } => write!(f, "{} | {}", left, right),
            Self::BitwiseShiftLeft { left, right } => write!(f, "{} << {}", left, right),
            Self::BitwiseShiftRight { left, right } => write!(f, "{} >> {}", left, right),
            Self::Cmp { left, right } => write!(f, "{} == {}", left, right),
            Self::NotCmp { left, right } => write!(f, "{} != {}", left, right),
            Self::Gt { left, right } => write!(f, "{} > {}", left, right),
            Self::Lt { left, right } => write!(f, "{} < {}", left, right),
            Self::LtEq { left, right } => write!(f, "{} <= {}", left, right),
            Self::GtEq { left, right } => write!(f, "{} >= {}", left, right),
            Self::Add { left, right } => write!(f, "{} + {}", left, right),
            Self::Sub { left, right } => write!(f, "{} - {}", left, right),
            Self::Mul { left, right } => write!(f, "{} * {}", left, right),
            Self::Div { left, right } => write!(f, "{} / {}", left, right),
            Self::Mod { left, right } => write!(f, "{} % {}", left, right),
            Self::Neg { expr } => write!(f, "-{}", expr),
            Self::Not { expr } => write!(f, "!{}", expr),
            Self::Tilde { expr } => write!(f, "~{}", expr),
            Self::Cast { kind, expr } => write!(f, "[{}]{}", kind, expr),
            Self::Ident(n) => write!(f, "n"),
            Self::StructLiteral { name, fields } => write!(f, "{} with ... end", name),
            Self::Char(n) => write!(f, "{n}"),
            Self::String(n) => write!(f, "{n}"),
            Self::Nil => write!(f, "Nil"),
            Self::False => write!(f, "False"),
            Self::This => write!(f, "This"),
            Self::True => write!(f, "True"),
            Self::Int(n) => write!(f, "{n}"),
            Self::Float(n) => write!(f, "{n}"),
            Self::FuncCall { name, params } => write!(f, "{}(...)", name),
            Self::Call { func, args } => write!(f, "{}(...)", func),
            Self::Member { object, name } => write!(f, "{}::{}", object, name),
            Self::Index { array, index } => write!(f, "{}[{}]", array, index),
            Self::Deref(n) => write!(f, "*{}", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StructFieldInit {
    pub identifier: String,
    pub value: Expr,
}

impl Expr {
    pub fn name(&self) -> Option<String> {
        match self {
            Self::Ident(s) => Some(match s.0.last() {
                Some(s) => s.clone(),
                None => return None,
            }),
            Self::String(s) => Some(s.clone()),
            _ => None,
        }
    }
    pub fn path(&self) -> Option<String> {
        match self {
            Self::Ident(s) => {
                let mut str = String::new();
                for (i, each) in s.0.iter().enumerate() {
                    if i < s.0.len().overflowing_sub(2).0 && s.0.len().overflowing_sub(2).1 == false
                    {
                        write!(str, "{}::", each);
                    } else if i == s.0.len().overflowing_sub(2).0
                        && s.0.len().overflowing_sub(2).1 == false
                    {
                        write!(str, "{}", each);
                    }
                }
                if str.len() == 0 { None } else { Some(str) }
            }
            _ => None,
        }
    }
    pub fn to_string(&self) -> Option<String> {
        match self {
            Self::Ident(s) => {
                let path = self.path().unwrap_or("".to_string());
                let name = match self.name() {
                    Some(s) => s,
                    None => return None,
                };
                if path.len() > 0 {
                    Some(format!("{}::{}", path, name))
                } else {
                    Some(name)
                }
            }
            Self::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QualifiedName(Vec<String>);

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("::"))
    }
}

impl QualifiedName {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(vec![s.into()])
    }

    pub fn push(&mut self, s: impl Into<String>) {
        self.0.push(s.into());
    }
}

impl Into<QualifiedName> for String {
    fn into(self) -> QualifiedName {
        QualifiedName(vec![self])
    }
}
