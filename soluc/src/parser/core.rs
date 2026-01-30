use crate::lexer;
use crate::lexer::Token::Identifier as Ident;
use crate::parser::types::*;

const EOF: lexer::Token = lexer::Token::Special(lexer::Special::Eof);
const TERM: lexer::Token = lexer::Token::Delim(lexer::Delimeter::Term);
const ENUM: lexer::Token = lexer::Token::Keyword(lexer::Keyword::Enum);
const CONST: lexer::Token = lexer::Token::Keyword(lexer::Keyword::Const);
const STRUCT: lexer::Token = lexer::Token::Keyword(lexer::Keyword::Struct);
const SWITCH: lexer::Token = lexer::Token::Keyword(lexer::Keyword::Switch);
const IF: lexer::Token = lexer::Token::Keyword(lexer::Keyword::If);
const THEN: lexer::Token = lexer::Token::Keyword(lexer::Keyword::Then);
const IS: lexer::Token = lexer::Token::Keyword(lexer::Keyword::Is);
const NAMESPACE: lexer::Token = lexer::Token::Keyword(lexer::Keyword::Namespace);
const LPAREN: lexer::Token = lexer::Token::Delim(lexer::Delimeter::Lparen);
const COLON: lexer::Token = lexer::Token::Delim(lexer::Delimeter::Colon);

#[derive(Debug, Clone)]
pub struct Parser {
    pub pos: usize,
    pub tokens: Vec<lexer::Token>,
    pub ast: Namespace,
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
        if self.tokens.len() == 0 {
            Ok(self.ast.clone())
        } else {
            self.ast.nodes = self.parse_p()?;
            Ok(self.ast.clone())
        }
    }

    // P ::= decl_list
    fn parse_p(&mut self) -> Result<Vec<Decl>, ParserError> {
        self.parse_decl_list()
    }

    // decl_list ::= D (TERM D)*
    fn parse_decl_list(&mut self) -> Result<Vec<Decl>, ParserError> {
        let mut list = Vec::new();
        list.push(self.parse_d()?);
        while self.matches(&TERM) {
            list.push(self.parse_d()?);
        }
        Ok(list)
    }

    // D ::= struct_D | enum_D | const_D | func_D | method_D | namespace_D
    fn parse_d(&mut self) -> Result<Decl, ParserError> {
        match self.peek() {
            Ident(s) => self.parse_func_and_method_d(s),
            ENUM => self.parse_enum_d(),
            CONST => self.parse_const_d(),
            STRUCT => self.parse_struct_d(),
            NAMESPACE => self.parse_namespace_d(),
            _ => Err(ParserError::new(self, "Invalid Declaration")),
        }
    }

    // struct_D ::= "struct identifier "is" TERM? member_list_D TERM? "end"
    fn parse_struct_d(&mut self) -> Result<Decl, ParserError> {
        todo!()
    }

    // enum_D ::= "enum" identifier "is" TERM? variant_list TERM? "end"
    fn parse_enum_d(&mut self) -> Result<Decl, ParserError> {
        todo!()
    }
    // const_d ::= "const" identifier ":" type "=" expr
    fn parse_const_d(&mut self) -> Result<Decl, ParserError> {
        todo!()
    }

    //func_D ::= identifier "(" params? ")" (":" type)? stmt_block
    //method_D ::= identifier "::" identifier "(" param_list? ")" (":" type)? stmt_block
    fn parse_func_and_method_d(&mut self, val: String) -> Result<Decl, ParserError> {
        self.advance();
        match self.peek() {
            LPAREN => {}
            COLON if self.next() == COLON => {
                self.advance();
                self.advance();
                let 
            }
            _ => Err(ParserError::new(self, "Invalid Function/Method Structure")),
        }
        todo!()
    }

    //namespace_D ::= "namespace" identifier "is TERM? decl_list TERM? "end"
    fn parse_namespace_d(&mut self) -> Result<Decl, ParserError> {
        todo!()
    }
}

// ---------------------------------------
// Helper Functions
// ---------------------------------------
impl Parser {
    fn peek(&self) -> lexer::Token {
        if self.pos < self.tokens.len() {
            self.tokens.get(self.pos).unwrap().clone()
        } else {
            EOF
        }
    }

    fn previous(&self) -> lexer::Token {
        if self.pos == 0 {
            EOF
        } else if (self.pos - 1) < self.tokens.len() {
            self.tokens.get(self.pos - 1).unwrap().clone()
        } else {
            EOF
        }
    }

    fn next(&self) -> lexer::Token {
        if (self.pos + 1) < self.tokens.len() {
            self.tokens.get(self.pos + 1).unwrap().clone()
        } else {
            EOF
        }
    }

    fn check(&self, tok: &lexer::Token) -> bool {
        *tok == self.peek()
    }

    fn advance(&mut self) -> lexer::Token {
        let tok = self.peek();
        self.pos += 1;
        tok
    }

    fn matches(&mut self, tok: &lexer::Token) -> bool {
        if self.check(tok) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn expect(&self, tok: &lexer::Token) -> Result<(), ParserError> {
        if *tok != self.peek() {
            Err(ParserError::new(
                self,
                format!("expected symble {}, got {}", *tok, self.peek()),
            ))
        } else {
            Ok(())
        }
    }
}
