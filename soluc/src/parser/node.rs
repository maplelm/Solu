use super::core::Type;
use super::expresions::Expr;
use super::statements::Block;
use crate::lexer::Token;
use crate::parser::error::ParseError;

#[derive(Debug, Clone)]
pub enum Node {
    Function(Function),
    Struct(StructDef),
    Enum(EnumDef),
    Const(ConstDef),
    TypeAlias(TypeAlias),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub kind: Type,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub kind: Type,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub fields: Option<Vec<Type>>,
}

#[derive(Debug, Clone)]
pub struct ConstDef {
    pub name: String,
    pub kind: Type,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub name: String,
    pub kind: Type,
}

// -------------------------------------------------
// Implementation of Nodes in Parser
// -------------------------------------------------

impl crate::parser::core::Parser {
    pub fn parse_struct(&mut self) -> Result<Node, ParseError> {
        todo!()
    }

    pub fn parse_enum(&mut self) -> Result<Node, ParseError> {
        todo!()
    }

    pub fn parse_const(&mut self) -> Result<Node, ParseError> {
        todo!()
    }

    pub fn parse_function(&mut self) -> Result<Node, ParseError> {
        let name = self.expected_identifier()?;
        self.expected_token(&Token::Lparen)?;
        let params = self.parse_params()?;
        self.expected_token(&Token::Rparen)?;
        let ret_type = if self.match_token(&Token::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expected_token(&Token::Do)?;
        let body = self.parse_block()?;
        self.expected_token(&Token::End)?;
        Ok(Node::Function(Function {
            name,
            params,
            ret_type,
            body,
        }))
    }
}

// -------------------------------------------------------
// Implementation of Node helper functions
// -------------------------------------------------------

impl crate::parser::core::Parser {
    fn parse_params(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params = Vec::new();
        if self.check(&Token::Rparen) {
            return Ok(params);
        }
        loop {
            let name = self.expected_identifier()?;
            self.expected_token(&Token::Colon)?;
            let kind = self.parse_type()?;
            params.push(Param { name, kind });
            if !self.match_token(&Token::Comma) {
                break;
            }
        }
        return Ok(params);
    }
}
