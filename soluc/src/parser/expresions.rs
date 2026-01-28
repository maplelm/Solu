use super::ops::*;
use crate::lexer::Token;
use crate::parser::core::{PrimitiveType, Type};
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
        let base = self.parse_type_base()?;
        let t: Type;
        if self.match_token(&Token::Lbracket) {
            let size: u64 = self.expected_int_literal()?;
            self.expected_token(&Token::Rbracket)?;
            t = Type::Array {
                elem: Box::new(base),
                size: size as usize,
            }
        } else {
            t = base;
        }
        Ok(t)
    }
    pub fn parse_type_base(&mut self) -> Result<Type, ParseError> {
        match self.peek() {
            Token::Star => {
                self.advance();
                let inner = self.parse_type()?;
                Ok(Type::Ptr(Box::new(inner)))
            }
            Token::Amp => {
                self.advance();
                let inner = self.parse_type()?;
                Ok(Type::Ref(Box::new(inner)))
            }
            Token::Ident(s) => {
                self.advance();
                Ok(Type::Named(s))
            }
            _ => {
                let result = Ok(Type::Primitive(self.parse_primitive_type()?));
                self.advance();
                return result;
            }
        }
    }

    pub fn parse_primitive_type(&mut self) -> Result<PrimitiveType, ParseError> {
        match self.peek() {
            Token::TypeI8 => Ok(PrimitiveType::I8),
            Token::TypeI16 => Ok(PrimitiveType::I16),
            Token::TypeI32 => Ok(PrimitiveType::I32),
            Token::TypeI64 => Ok(PrimitiveType::I64),
            Token::TypeU8 => Ok(PrimitiveType::U8),
            Token::TypeU16 => Ok(PrimitiveType::U16),
            Token::TypeU32 => Ok(PrimitiveType::U32),
            Token::TypeU64 => Ok(PrimitiveType::U64),
            Token::TypeF32 => Ok(PrimitiveType::F32),
            Token::TypeF64 => Ok(PrimitiveType::F64),
            Token::TypeChar => Ok(PrimitiveType::Char),
            Token::TypeBool => Ok(PrimitiveType::Bool),
            t => Err(ParseError::new(format!(
                "Expected Primitive Type got: {:?}",
                t
            ))),
        }
    }
}
