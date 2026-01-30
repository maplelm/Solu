use crate::lexer;
use std::fmt;

pub type Program = Vec<lexer::Token>;
pub type Ast = Vec<Namespace>;

#[derive(Debug, Clone)]
pub struct Namespace {
    pub name: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Function,
    Struct,
    Enum,
    Namespace,
    Const,
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
