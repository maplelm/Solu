use super::ops::*;
use crate::parser::core::Type;
use crate::parser::error::ParseError;

#[derive(Debug, Clone)]
pub enum Expr {
    Int(u64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
    Nil,
    Ident(String),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
    Field {
        expr: Box<Expr>,
        field: String,
    },
    Deref(Box<Expr>),
    AddrOf(Box<Expr>),
    StructLit {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    ArrayLit(Vec<Expr>),
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },
}

// --------------------------------------------
// Expression implementation for Parser
// --------------------------------------------

impl crate::parser::core::Parser {
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        todo!()
    }

    pub fn parse_type(&mut self) -> Result<Type, ParseError> {
        todo!()
    }
}
