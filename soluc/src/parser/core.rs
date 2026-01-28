use super::super::lexer::*;
use super::node::*;
use crate::parser::error::ParseError;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Named(String),
    Ptr(Box<Type>),
    Ref(Box<Type>),
    Array { elem: Box<Type>, size: usize },
    Slice(Box<Type>),
    Inferred,
}

pub struct Parser {
    pub tokens: Vec<Token>,
    paren_depth: usize,
    bracket_depth: usize,
    brace_depth: usize,
    pub pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            paren_depth: 0,   // ()
            bracket_depth: 0, // []
            brace_depth: 0,   // {}
            pos: 0,
        }
    }

    // Entry Point into the parser
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();
        while !self.is_at_end() {
            items.push(self.parse_item()?);
        }
        Ok(Program { items })
    }

    pub fn parse_item(&mut self) -> Result<Node, ParseError> {
        match self.peek() {
            Token::Struct => self.parse_struct(),
            Token::Enum => self.parse_enum(),
            Token::Const => self.parse_const(),
            Token::Type => self.parse_type_alias(),
            Token::Ident(_) => self.parse_function(),
            _ => Err(ParseError::new(format!(
                "expected top-level item, found {:?}",
                self.peek(),
            ))),
        }
    }
    // ------------------------------------------
    // Expected
    // ------------------------------------------

    pub fn expected_identifier(&mut self) -> Result<String, ParseError> {
        match self.peek() {
            Token::Ident(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(ParseError::new(format!(
                "Expected identifier, found {:?}",
                self.peek()
            ))),
        }
    }

    pub fn expected_token(&mut self, token: &Token) -> Result<Token, ParseError> {
        if *token == self.peek() {
            Ok(self.advance())
        } else {
            Err(ParseError::new(format!(
                "Expected {:?}, found {:?}",
                token,
                self.peek()
            )))
        }
    }

    // ------------------------------------------
    // Helper Functions
    // ------------------------------------------

    pub fn check(&self, expected: &Token) -> bool {
        return *expected == self.peek();
    }

    pub fn peek(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof)
    }

    pub fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
        }
        return self.check(token);
    }

    pub fn advance_raw(&mut self) -> Token {
        let tok = self.peek();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        return tok;
    }

    pub fn advance(&mut self) -> Token {
        let tok = self.peek();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        match &tok {
            Token::Lparen => self.paren_depth += 1,
            Token::Rparen => self.paren_depth -= 1,
            Token::Lbracket => self.bracket_depth += 1,
            Token::Rbracket => self.bracket_depth -= 1,
            Token::Lbrace => self.bracket_depth += 1,
            Token::Rbrace => self.bracket_depth -= 1,
            _ => {}
        }
        return tok;
    }

    pub fn bracket_incased(&self) -> bool {
        self.paren_depth > 0 || self.brace_depth > 0 || self.bracket_depth > 0
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
