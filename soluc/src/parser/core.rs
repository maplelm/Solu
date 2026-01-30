use crate::lexer;
use crate::parser::types::*;

#[derive(Debug, Clone)]
pub struct Parser {
    pos: usize,
    tokens: Vec<lexer::Token>,
    ast: Namespace,
}

impl Parser {
    pub fn new(tokens: Program) -> Self {
        Self {
            pos: 0,
            tokens,
            ast: Namespace {
                name: "Global".to_string(),
                nodes: vec![],
            },
        }
    }

    pub fn parse(&mut self) -> Result<Namespace, ParserError> {
        todo!()
    }
}
