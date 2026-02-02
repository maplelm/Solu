#![allow(unused)]
use crate::lexer;
use crate::lexer::Token;
use crate::parser::types::*;

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
        while self.matches(&Token::TERM) {
            list.push(self.parse_d()?);
        }
        Ok(list)
    }

    // D ::= struct_D | enum_D | const_D | func_D | method_D | namespace_D
    fn parse_d(&mut self) -> Result<Decl, ParserError> {
        match self.peek() {
            Token::Identifier(s) => self.parse_func_and_method_d(s),
            Token::KEY_ENUM => self.parse_enum_d(),
            Token::KEY_CONST => self.parse_const_d(),
            Token::KEY_STRUCT => self.parse_struct_d(),
            Token::KEY_NAMESPACE => self.parse_namespace_d(),
            _ => Err(ParserError::new(self, "Invalid Declaration")),
        }
    }

    // struct_D ::= "struct identifier "is" TERM? member_list_D TERM? "end"
    fn parse_struct_d(&mut self) -> Result<Decl, ParserError> {
        self.expect(&Token::KEY_STRUCT)?;
        let name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => return Err(ParserError::new(self, "Failed to parse struct name")),
        };
        self.expect(&Token::KEY_IS)?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        let mut list = vec![];
        let mut ident = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => return Err(ParserError::new(self, "Failed to parse struct field name")),
        };
        self.expect(&Token::COLON)?;
        let mut t = self.parse_type()?;
        list.push(StructFieldDecl {
            name: ident,
            kind: t,
        });

        while self.matches(&Token::TERM) {
            if self.check(&Token::KEY_END) {
                break;
}
            ident = match self.expect(&Token::IDENTIFER)?.to_string() {
                Some(s) => s,
                None => return Err(ParserError::new(self, "Failed to get struct field name")),
            };
            self.expect(&Token::COLON)?;
            t = self.parse_type()?;
            list.push(StructFieldDecl {
                name: ident,
                kind: t,
            });
        }
        self.expect(&Token::KEY_END)?;
        Ok(Decl::Struct {
            name: name,
            fields: list,
        })
    }

    // enum_D ::= "enum" identifier "is" TERM? variant_list TERM? "end"
    fn parse_enum_d(&mut self) -> Result<Decl, ParserError> {
        self.expect(&Token::KEY_ENUM)?;
        let name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => return Err(ParserError::new(self, "Failed to parse enum name")),
        };
        self.expect(&Token::KEY_IS)?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        let mut variants = vec![match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => return Err(ParserError::new(self, "Failed to parse enum variant")),
        }];
        while self.matches(&Token::TERM) {
            if self.check(&Token::KEY_END) {
                break;
            }
            variants.push(match self.expect(&Token::IDENTIFER)?.to_string() {
                Some(s) => s,
                None => return Err(ParserError::new(self, "Failed to parse enum variant")),
            });
        }
        self.expect(&Token::KEY_END)?;
        Ok(Decl::Enum {
            name: name,
            variants: variants,
        })
    }
    // const_d ::= "const" identifier ":" type "=" expr
    fn parse_const_d(&mut self) -> Result<Decl, ParserError> {
        self.expect(&Token::KEY_CONST)?;
        let name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => return Err(ParserError::new(self, "Failed to parse const name")),
        };
        self.expect(&Token::COLON)?;
        let t = self.parse_type()?;
        self.expect(&Token::OP_EQ)?;
        let val = self.parse_expr()?;
        Ok(Decl::Const {
            name: name,
            kind: t,
            value: val,
        })
    }

    //func_D ::= identifier "(" params? ")" (":" type)? stmt_block
    //method_D ::= identifier "::" identifier "(" param_list? ")" (":" type)? stmt_block
    fn parse_func_and_method_d(&mut self, val: String) -> Result<Decl, ParserError> {
        self.advance();
        match self.peek() {
            // Function
            Token::LPAREN => {
                let name = match self.previous().to_string() {
                    Some(s) => s,
                    None => return Err(ParserError::new(self, "Invalid Function Name")),
                };
                self.advance();
                let params: Option<Vec<Param>>;
                if !self.matches(&Token::RPAREN) {
                    params = Some(self.parse_param_list()?);
                    self.expect(&Token::RPAREN)?;
                } else {
                    params = None;
                }
                let mut t: Option<Type> = None;
                if self.matches(&Token::COLON) {
                    t = Some(self.parse_type()?);
                }
                let stmts = self.parse_stmt_block()?;
                Ok(Decl::Function {
                    name: name,
                    params: params.unwrap_or(vec![]),
                    ret_type: t,
                    body: stmts,
                })
            }
            // Member Function
            Token::COLON if self.next() == Token::COLON => {
                let parent = match self.previous().to_string() {
                    Some(s) => s,
                    None => return Err(ParserError::new(self, "Invalid Member Parent Name")),
                };
                self.advance();
                self.advance();
                let name = match self.expect(&Token::IDENTIFER)?.to_string() {
                    Some(s) => s,
                    None => return Err(ParserError::new(self, "Invalid Member Name")),
                };
                self.expect(&Token::LPAREN)?;
                let params: Option<Vec<Param>>;
                if !self.matches(&Token::RPAREN) {
                    params = Some(self.parse_param_list()?);
                    self.expect(&Token::RPAREN)?;
                } else {
                    params = None;
                }
                let mut t: Option<Type> = None;
                if self.matches(&Token::COLON) {
                    t = Some(self.parse_type()?);
                }
                let stmts = self.parse_stmt_block()?;
                Ok(Decl::Member {
                    name: name,
                    parent: parent,
                    params: params.unwrap_or(vec![]),
                    ret_type: t,
                    body: stmts,
                })
            }
            // Unexpected Token
            _ => Err(ParserError::new(
                self,
                "Unexpected Function or Member Token",
            )),
        }
    }

    //namespace_D ::= "namespace" identifier "is TERM? decl_list TERM? "end"
    fn parse_namespace_d(&mut self) -> Result<Decl, ParserError> {
        self.expect(&Token::KEY_NAMESPACE)?;
        let name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => return Err(ParserError::new(self, "Failed to parse namespace name")),
        };
        self.expect(&Token::KEY_IS)?;
        let declares = self.parse_decl_list()?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        self.expect(&Token::KEY_END)?;
        Ok(Decl::Namespace(Namespace { name: name, nodes: declares })
    }

    fn parse_stmt_block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        todo!()
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParserError> {
        let mut list = vec![self.parse_param()?];
        while self.matches(&Token::COMMA) {
            list.push(self.parse_param()?);
        }
        self.expect(&Token::RPAREN)?;
        Ok(list)
    }

    fn parse_param(&mut self) -> Result<Param, ParserError> {
        let name: String = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => {
                return Err(ParserError::new(
                    self,
                    "Expected Identifier while parsing param",
                ));
            }
        };
        self.expect(&Token::COLON)?;
        let kind = self.parse_type()?;
        Ok(Param { name, kind })
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let base = self.parse_type_base()?;
        if self.matches(&Token::LBRACKET) {
            if self.matches_any(&[Token::INT, Token::IDENTIFER]) {
                let size = match self.previous() {
                    Token::Num(lexer::NumLiteral::Int(n)) => ArraySize::Usize(n as usize),
                    Token::Identifier(s) => ArraySize::Identifier(s),
                    _ => return Err(ParserError::new(self, "improper array specificiation")),
                };
                self.expect(&Token::RBRACKET)?;
                Ok(Type::Array {
                    kind: base,
                    size: size,
                })
            } else {
                Err(ParserError::new(self, "improper array specificiation"))
            }
        } else {
            Ok(Type::Base(base))
        }
    }

    fn parse_type_base(&mut self) -> Result<TypeBase, ParserError> {
        match self.peek() {
            Token::OP_STAR => {
                self.advance();
                Ok(TypeBase::Ptr(Box::new(self.parse_type()?)))
            }
            Token::OP_AMP => {
                self.advance();
                Ok(TypeBase::Ref(Box::new(self.parse_type()?)))
            }
            Token::Identifier(s) => {
                self.advance();
                Ok(TypeBase::Ident(s))
            }
            _ => Ok(TypeBase::Prim(self.parse_primitive_type()?)),
        }
    }

    fn parse_primitive_type(&mut self) -> Result<TypePrim, ParserError> {
        let t = match self.peek() {
            Token::Keyword(k) => match k {
                lexer::Keyword::TypeI8 => TypePrim::I8,
                lexer::Keyword::TypeI16 => TypePrim::I16,
                lexer::Keyword::TypeI32 => TypePrim::I32,
                lexer::Keyword::TypeI64 => TypePrim::I64,
                lexer::Keyword::TypeU8 => TypePrim::U8,
                lexer::Keyword::TypeU16 => TypePrim::U16,
                lexer::Keyword::TypeU32 => TypePrim::U32,
                lexer::Keyword::TypeU64 => TypePrim::U64,
                lexer::Keyword::TypeF32 => TypePrim::F32,
                lexer::Keyword::TypeF64 => TypePrim::F64,
                lexer::Keyword::TypeChar => TypePrim::Char,
                lexer::Keyword::TypeBool => TypePrim::Bool,
                _ => return Err(ParserError::new(self, "unexpected keyword")),
            },
            _ => return Err(ParserError::new(self, "invalid primitive type")),
        };
        self.advance();
        Ok(t)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_ternary_expr()
    }

    fn parse_ternary_expr(&mut self) -> Result<Expr, ParserError> {
        let a = self.parse_logical_or_expr()?;
        if self.matches(&Token::OP_TERINARY) {
            let b = self.parse_expr()?;
            self.expect(&Token::COLON)?;
            let c = self.parse_ternary_expr()?;
            Ok(Expr::Ternary {
                a: Box::new(a),
                b: Box::new(b),
                c: Box::new(c),
            })
        } else {
            Ok(a)
        }
    }

    fn parse_logical_or_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_logical_and_expr()?;
        while self.matches(&Token::OP_LOR) {
            left = Expr::LogicOr {
                left: Box::new(left),
                right: Box::new(self.parse_logical_and_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_logical_and_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_bitwise_xor_expr()?;
        while self.matches(&Token::OP_LOR) {
            left = Expr::LogicOr {
                left: Box::new(left),
                right: Box::new(self.parse_bitwise_xor_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_or_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_bitwise_xor_expr()?;
        while self.matches(&Token::OP_LOR) {
            left = Expr::LogicOr {
                left: Box::new(left),
                right: Box::new(self.parse_bitwise_xor_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_xor_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_bitwise_and_expr()?;
        while self.matches(&Token::OP_LOR) {
            left = Expr::LogicOr {
                left: Box::new(left),
                right: Box::new(self.parse_bitwise_and_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_and_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_bitwise_shift_expr()?;
        while self.matches(&Token::OP_LOR) {
            left = Expr::LogicOr {
                left: Box::new(left),
                right: Box::new(self.parse_bitwise_shift_expr()?),
            };
        }
        Ok(left)
    }

    fn parse_bitwise_shift_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_comparison_expr()?;
        while self.matches_any(&[Token::OP_GTGT, Token::OP_LTLT]) {
            let op = self.previous();
            left = match op {
                Token::OP_GTGT => Expr::BitwiseShiftRight {
                    left: Box::new(left),
                    right: Box::new(self.parse_comparison_expr()?),
                },
                Token::OP_LTLT => Expr::BitwiseShiftLeft {
                    left: Box::new(left),
                    right: Box::new(self.parse_comparison_expr()?),
                },
                _ => unreachable!(),
            }
        }
        Ok(left)
    }

    fn parse_comparison_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_additive_expr()?;
        while self.matches_any(&[
            Token::OP_EQEQ,
            Token::OP_NOTEQ,
            Token::OP_GT,
            Token::OP_LT,
            Token::OP_LTEQ,
            Token::OP_GTEQ,
        ]) {
            let op = self.previous();
            left = match op {
                Token::OP_EQEQ => Expr::Cmp {
                    left: Box::new(left),
                    right: Box::new(self.parse_additive_expr()?),
                },
                Token::OP_NOTEQ => Expr::NotCmp {
                    left: Box::new(left),
                    right: Box::new(self.parse_additive_expr()?),
                },
                Token::OP_GT => Expr::Gt {
                    left: Box::new(left),
                    right: Box::new(self.parse_additive_expr()?),
                },
                Token::OP_LT => Expr::Lt {
                    left: Box::new(left),
                    right: Box::new(self.parse_additive_expr()?),
                },
                Token::OP_LTEQ => Expr::LtEq {
                    left: Box::new(left),
                    right: Box::new(self.parse_additive_expr()?),
                },
                Token::OP_GTEQ => Expr::GtEq {
                    left: Box::new(left),
                    right: Box::new(self.parse_additive_expr()?),
                },
                _ => unreachable!(),
            }
        }
        Ok(left)
    }

    fn parse_additive_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_multiplicative_expr()?;
        while self.matches_any(&[Token::OP_PLUS, Token::OP_MINUS]) {
            let op = self.previous();
            left = match op {
                Token::OP_PLUS => Expr::Add {
                    left: Box::new(left),
                    right: Box::new(self.parse_multiplicative_expr()?),
                },
                Token::OP_MINUS => Expr::Sub {
                    left: Box::new(left),
                    right: Box::new(self.parse_multiplicative_expr()?),
                },
                _ => unreachable!(),
            }
        }
        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary_expr()?;
        while self.matches_any(&[Token::OP_STAR, Token::OP_DIV, Token::OP_MOD]) {
            let op = self.previous();
            left = match op {
                Token::OP_STAR => Expr::Mul {
                    left: Box::new(left),
                    right: Box::new(self.parse_unary_expr()?),
                },
                Token::OP_DIV => Expr::Div {
                    left: Box::new(left),
                    right: Box::new(self.parse_unary_expr()?),
                },
                Token::OP_MOD => Expr::Mod {
                    left: Box::new(left),
                    right: Box::new(self.parse_unary_expr()?),
                },
                _ => unreachable!(),
            }
        }
        Ok(left)
    }

    fn parse_unary_expr(&mut self) -> Result<Expr, ParserError> {
        match self.peek() {
            Token::OP_MINUS => {
                self.advance();
                Ok(Expr::Neg {
                    expr: Box::new(self.parse_unary_expr()?),
                })
            }
            Token::OP_STAR => {
                self.advance();
                Ok(Expr::Deref(Box::new(self.parse_unary_expr()?)))
            }
            Token::OP_NOT => {
                self.advance();
                Ok(Expr::Not {
                    expr: Box::new(self.parse_unary_expr()?),
                })
            }
            Token::OP_TILDE => {
                self.advance();
                Ok(Expr::Tilde {
                    expr: Box::new(self.parse_unary_expr()?),
                })
            }
            // type_cast_expr
            Token::LBRACKET => self.parse_type_case_expr(),
            // postfix_expr
            _ => self.parse_postfix_expr(),
        }
    }

    fn parse_type_case_expr(&mut self) -> Result<Expr, ParserError> {
        self.advance();
        let t = self.parse_type()?;
        self.expect(&Token::RBRACKET)?;
        Ok(Expr::Cast {
            kind: t,
            expr: Box::new(self.parse_unary_expr()?),
        })
    }

    fn parse_postfix_expr(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_primary_expr()?;
        // postfix_op
        while self.matches_any(&[Token::LPAREN, Token::OP_MEMBERACCESSOR, Token::LBRACKET]) {
            let sym = self.previous();
            expr = match sym {
                Token::LPAREN => {
                    self.advance();
                    let mut args = vec![self.parse_expr()?];
                    while self.matches(&Token::COMMA) {
                        args.push(self.parse_expr()?);
                    }
                    self.expect(&Token::RPAREN)?;
                    Expr::Call {
                        func: Box::new(expr),
                        args: args,
                    }
                }
                Token::OP_MEMBERACCESSOR => Expr::Member {
                    object: Box::new(expr),
                    name: self.expect(&Token::IDENTIFER)?.to_string().unwrap(),
                },
                Token::LBRACKET => {
                    let index = self.parse_expr()?;
                    self.expect(&Token::RBRACKET)?;
                    Expr::Index {
                        array: Box::new(expr),
                        index: Box::new(index),
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(expr)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, ParserError> {
        match self.peek() {
            Token::Num(n) => {
                self.advance();
                match n {
                    lexer::NumLiteral::Int(n) => Ok(Expr::Int(n)),
                    lexer::NumLiteral::Float(n) => Ok(Expr::Float(n)),
                }
            }
            Token::KEY_TRUE => {
                self.advance();
                Ok(Expr::True)
            }
            Token::KEY_FALSE => {
                self.advance();
                Ok(Expr::False)
            }
            Token::KEY_NIL => {
                self.advance();
                Ok(Expr::Nil)
            }
            Token::Str(s) => {
                self.advance();
                Ok(Expr::String(s))
            }
            Token::Identifier(s) => Ok(Expr::Ident(s)),
            Token::LPAREN => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RPAREN)?;
                Ok(expr)
            }
            _ => Err(ParserError::new(
                self,
                "unexpected token when parsing unary expression",
            )),
        }
    }

    // assign_block == struct_literal
    fn parse_struct_literal(&mut self) -> Result<Expr, ParserError> {
        let name = self.previous();
        if self.next() == Token::TERM {
            self.advance();
            self.advance();
        } else {
            self.advance();
        }
        Ok(Expr::StructLiteral {
            name: name.to_string().unwrap(),
            fields: self.parse_field_list()?,
        })
    }

    fn parse_field_list(&mut self) -> Result<Vec<StructFieldInit>, ParserError> {
        let mut list = vec![self.parse_field_init()?];
        while self.matches(&Token::TERM) {
            if self.check(&Token::KEY_END) {
                break;
            }
            list.push(self.parse_field_init()?);
        }
        self.expect(&Token::KEY_END);
        Ok(list)
    }

    fn parse_field_init(&mut self) -> Result<StructFieldInit, ParserError> {
        let name = self.expect(&Token::IDENTIFER)?;
        self.expect(&Token::OP_EQ)?;
        let value = self.parse_expr()?;
        Ok(StructFieldInit {
            identifier: name.to_string().unwrap(),
            value: value,
        })
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
            Token::EOF
        }
    }

    fn previous(&self) -> lexer::Token {
        if self.pos == 0 {
            Token::EOF
        } else if (self.pos - 1) < self.tokens.len() {
            self.tokens.get(self.pos - 1).unwrap().clone()
        } else {
            Token::EOF
        }
    }

    fn next(&self) -> lexer::Token {
        if (self.pos + 1) < self.tokens.len() {
            self.tokens.get(self.pos + 1).unwrap().clone()
        } else {
            Token::EOF
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

    fn matches_any(&mut self, toks: &[lexer::Token]) -> bool {
        if toks.iter().any(|a| self.check(a)) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn matches(&mut self, tok: &lexer::Token) -> bool {
        if self.check(tok) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    fn expect(&self, tok: &lexer::Token) -> Result<lexer::Token, ParserError> {
        let t = self.peek();
        if *tok != t {
            Err(ParserError::new(
                self,
                format!("expected symble {}, got {}", *tok, self.peek()),
            ))
        } else {
            Ok(t)
        }
    }
}
