use super::super::lexer::*;
use super::node::*;
use crate::parser::error::ParseError;
use crate::parser::statements::Stmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Array { elem: Box<Type>, size: usize },
    Named(String),
    Ptr(Box<Type>),
    Ref(Box<Type>),
    Primitive(PrimitiveType),
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
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
            items.push(self.parse_node()?);
        }
        Ok(Program { nodes: items })
    }

    pub fn parse_node(&mut self) -> Result<Node, ParseError> {
        match self.peek() {
            Token::Struct => self.parse_struct(),
            Token::Enum => self.parse_enum(),
            Token::Const => self.parse_const(),
            Token::Type => self.parse_type_alias(),
            Token::Ident(_) => self.parse_function(),
            _ => Err(ParseError::new(format!(
                "expected node token, found {:?}",
                self.peek(),
            ))),
        }
    }

    // ------------------------------------------
    // Node Parsing
    // ------------------------------------------

    pub fn prase_function(&mut self) -> Result<Node, ParseError> {
        // identifier
        let name = self.expected_identifier()?;
        // "("
        self.expected_token(&Token::Lparen)?;
        // param_list?
        let params = self.parse_param_list()?;
        // ")"
        self.expected_token(&Token::Rparen)?;
        // (":" type)?
        let return_type: Option<Type>;
        if self.match_token(&Token::Colon) {
            return_type = Some(self.parse_type()?);
        } else {
            return_type = None;
        }
        // "do"
        self.expected_token(&Token::Do)?;
        // TERM
        self.expected_token(&Token::Term)?;
        // stmt*
        let mut body: Vec<Stmt> = Vec::new();
        while !self.check(&Token::End) {
            body.push(self.parse_stmt()?);
        }
        // "end"
        self.expected_token(&Token::End)?;
        // TERM
        self.expected_token(&Token::Term)?;

        Ok(Node::Function(Function {
            name,
            params,
            ret_type: return_type,
            body,
        }))
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParseError> {
        todo!()
    }

    // ------------------------------------------
    // Expected
    // ------------------------------------------

    pub fn expected_int_literal(&mut self) -> Result<u64, ParseError> {
        match self.peek() {
            Token::Int(n) => {
                self.advance();
                Ok(n)
            }
            t => Err(ParseError::new(format!("Expcted int literal, got {:?}", t))),
        }
    }

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

    pub fn advance(&mut self) -> Token {
        let tok = self.peek();
        if self.pos < self.tokens.len() && self.tokens[self.pos] != Token::Eof {
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

    pub fn in_braces(&self) -> bool {
        self.paren_depth > 0 || self.brace_depth > 0 || self.bracket_depth > 0
    }

    pub fn is_end(&self) -> bool {
        self.tokens[self.pos] == Token::Eof || self.pos >= self.tokens.len()
    }
}
