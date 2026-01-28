use super::core::Type;
use super::expresions::Expr;
use super::ops::AssignOp;
use crate::lexer::Token;
use crate::parser::error::ParseError;

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl {
        name: String,
        kind: Type,
        value: Expr,
        mutable: bool,
    },
    Assign {
        target: Expr,
        op: AssignOp,
        value: Expr,
    },
    If {
        cond: Expr,
        body: Block,
        elif_blocks: Vec<(Expr, Block)>,
        else_block: Option<Block>,
    },
    While {
        cond: Expr,
        body: Block,
    },
    For {
        var: String,
        iter: Expr,
        body: Block,
    },
    Retrun(Option<Expr>),
    Break,
    Continue,
    Defer(Box<Stmt>),
    Arena {
        name: String,
        size: Option<Expr>,
    },
    Expr(Expr),
}

pub type Block = Vec<Stmt>;

// ----------------------------------------------
// Implementation of Statements in parser
// ----------------------------------------------

impl crate::parser::core::Parser {
    pub fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Token::Mut => self.parse_var_decl(true),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            Token::Return => self.parse_return(),
            Token::Break => {
                self.advance();
                Ok(Stmt::Break)
            }
            Token::Continue => {
                self.advance();
                Ok(Stmt::Continue)
            }
            Token::Defer => {
                self.advance();
                Ok(Stmt::Defer(Box::new(self.parse_stmt()?)))
            }
            Token::Arena => self.parse_arena(),
            Token::Ident(_) => self.parse_ident_stmt(),
            _ => Ok(Stmt::Expr(self.parse_expr()?)),
        }
    }

    pub fn parse_block(&mut self) -> Result<Block, ParseError> {
        let mut stmts = Vec::new();
        loop {
            if self.check(&Token::End)
                || self.check(&Token::Else)
                || self.check(&Token::Elif)
                || self.is_at_end()
            {
                break;
            }
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
            self.expected_token(&Token::Term)?;
        }
        return Ok(stmts);
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        todo!()
    }

    fn parse_ident_stmt(&mut self) -> Result<Stmt, ParseError> {
        todo!()
    }

    fn parse_arena(&mut self) -> Result<Stmt, ParseError> {
        todo!()
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        todo!()
    }

    fn parse_for(&mut self) -> Result<Stmt, ParseError> {
        todo!()
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        todo!()
    }

    fn parse_var_decl(&mut self, mutable: bool) -> Result<Stmt, ParseError> {
        if mutable {
            self.expected_token(&Token::Mut)?;
        }
        let name = self.expected_identifier()?;
        self.expected_token(&Token::Colon)?;
        let kind = self.parse_type()?;
        self.expected_token(&Token::Eq)?;
        let value = self.parse_expr()?;
        return Ok(Stmt::VarDecl {
            name,
            kind,
            value,
            mutable,
        });
    }
}
