#![allow(unused)]

use crate::lexer::{self, Span, Token};
use crate::parser::types::*;

#[derive(Debug, Clone)]
pub struct Parser {
    pub pos: usize,
    pub span_table: Vec<Span>,
    pub tokens: Vec<Token>,
    pub ast: Namespace,
}

impl Parser {
    pub fn new(tokens: Program, span_table: Vec<Span>) -> Self {
        Self {
            span_table,
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
            if !self.check(&Token::Eof) {
                list.push(self.parse_d()?);
            }
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
        #[cfg(debug_assertions)]
        println!("Parsing a struct...");
        self.expect(&Token::KEY_STRUCT)?;
        let name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => {
                return Err(ParserError::new(self, "Failed to parse struct name"));
            }
        };
        self.expect(&Token::KEY_IS)?;
        if self.check(&Token::TERM) {
            self.advance();
        }

        #[cfg(debug_assertions)]
        println!("\tParsing Declaration statements of Struct...");
        let body = self.parse_declare_list()?;

        #[cfg(debug_assertions)]
        println!("\t Expecting End of Struct...");
        self.expect(&Token::KEY_END)?;
        Ok(Decl::Struct {
            name: name,
            fields: body,
        })
    }

    fn parse_declare_list(&mut self) -> Result<Vec<StructFieldDecl>, ParserError> {
        #[cfg(debug_assertions)]
        println!("\tParsing first Delcaration statement...");
        let mut list = vec![self.parse_declare_member()?];
        while self.matches(&Token::TERM) {
            if self.peek() == Token::KEY_END {
                break;
            }

            #[cfg(debug_assertions)]
            println!("\tarsing Consecutive Delcaration...");
            list.push(self.parse_declare_member()?);
        }
        Ok(list)
    }

    fn parse_declare_member(&mut self) -> Result<StructFieldDecl, ParserError> {
        #[cfg(debug_assertions)]
        println!("\t\tGetting Identifier for Declaration...");
        let name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => {
                return Err(ParserError::new(
                    self,
                    "failed to get struct declare field name",
                ));
            }
        };

        #[cfg(debug_assertions)]
        println!("\t\tExpecting Identifer Type Seperator colon...");
        self.expect(&Token::COLON)?;
        #[cfg(debug_assertions)]
        println!("\t\tParsing type for declaration...");
        let t = self.parse_type()?;
        #[cfg(debug_assertions)]
        println!("\t\tReturning Field Declaration...");

        Ok(StructFieldDecl {
            name: name,
            kind: t,
        })
    }

    // enum_D ::= "enum" identifier "is" TERM? variant_list TERM? "end"
    fn parse_enum_d(&mut self) -> Result<Decl, ParserError> {
        self.expect(&Token::KEY_ENUM)?;
        let name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => {
                return Err(ParserError::new(self, "Failed to parse enum name"));
            }
        };
        self.expect(&Token::KEY_IS)?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        let mut variants = vec![match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => {
                return Err(ParserError::new(self, "Failed to parse enum variant"));
            }
        }];
        while self.matches(&Token::TERM) {
            if self.check(&Token::KEY_END) {
                break;
            }
            variants.push(match self.expect(&Token::IDENTIFER)?.to_string() {
                Some(s) => s,
                None => {
                    return Err(ParserError::new(self, "Failed to parse enum variant"));
                }
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
            None => {
                return Err(ParserError::new(self, "Failed to parse const name"));
            }
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
                    None => {
                        return Err(ParserError::new(self, "Invalid Function Name"));
                    }
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
                    None => {
                        return Err(ParserError::new(self, "Invalid Member Parent Name"));
                    }
                };
                self.advance();
                self.advance();
                let name = match self.expect(&Token::IDENTIFER)?.to_string() {
                    Some(s) => s,
                    None => {
                        return Err(ParserError::new(self, "Invalid Member Name"));
                    }
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
            None => {
                return Err(ParserError::new(self, "Failed to parse namespace name"));
            }
        };
        self.expect(&Token::KEY_IS)?;
        let declares = self.parse_decl_list()?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        self.expect(&Token::KEY_END)?;
        Ok(Decl::Namespace(Namespace {
            name: name,
            nodes: declares,
        }))
    }

    fn parse_stmt_block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        self.expect(&Token::KEY_DO)?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        let stmts = self.parse_stmt_list()?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        self.expect(&Token::KEY_END)?;
        Ok(stmts)
    }

    fn parse_stmt_list(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut stmt_list = vec![self.parse_stmt()?];
        while self.matches(&Token::TERM) {
            if matches!(
                self.peek(),
                Token::KEY_END
                    | Token::KEY_ELIF
                    | Token::KEY_ELSE
                    | Token::KEY_CASE
                    | Token::KEY_DEFAULT
            ) {
                break;
            }
            stmt_list.push(self.parse_stmt()?);
        }
        Ok(stmt_list)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        match self.peek() {
            // Init statment
            Token::KEY_MUT => self.parse_var_decl_stmt(),
            Token::Identifier(s) if self.next() == Token::COLON => self.parse_var_decl_stmt(),
            // Control Flow Statements
            Token::KEY_BREAK => self.parse_break_stmt(),
            Token::KEY_IF => self.parse_if_stmt(),
            Token::KEY_SWITCH => self.parse_switch_stmt(),
            Token::KEY_CONTINUE => self.parse_continue_stmt(),
            Token::KEY_WHILE => self.parse_while_stmt(),
            Token::KEY_FOR => self.parse_for_stmt(),
            // --
            Token::KEY_RETURN => self.parse_return_stmt(),
            // --
            _ => self.parse_expr_or_assign_stmt(),
        }
    }

    fn parse_expr_or_assign_stmt(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.parse_expr()?;
        if self.is_assign_op() {
            if !self.is_valid_lvalue(&expr) {
                return Err(ParserError::new(self, "Invalid assignment target"));
            }
            let op = self.parse_assign_op()?;
            let rvalue = self.parse_expr()?;
            Ok(Stmt::Assign {
                left: expr,
                op: op,
                right: rvalue,
            })
        } else {
            Ok(Stmt::Expr(expr))
        }
    }

    fn is_valid_lvalue(&mut self, expr: &Expr) -> bool {
        match expr {
            Expr::Ident(_) => true,
            Expr::This => true,
            Expr::Deref(inner) => self.is_valid_lvalue(inner),
            Expr::Member { object, name } => self.is_valid_lvalue(object),
            Expr::Index { array, index } => self.is_valid_lvalue(array),
            _ => false,
        }
    }

    fn parse_assign_op(&mut self) -> Result<AssignOp, ParserError> {
        let op = match self.peek() {
            Token::OP_EQ => AssignOp::Eq,
            Token::OP_PLUSEQ => AssignOp::PlusEq,
            Token::OP_SUBEQ => AssignOp::SubEq,
            Token::OP_STAREQ => AssignOp::MulEq,
            Token::OP_DIVEQ => AssignOp::DivEq,
            Token::OP_MODEQ => AssignOp::ModEq,
            Token::OP_AMPEQ => AssignOp::AndEq,
            Token::OP_PIPEEQ => AssignOp::OrEq,
            Token::OP_CARETEQ => AssignOp::XorEq,
            _ => {
                return Err(ParserError::new(self, "Expected assignment operator"));
            }
        };
        self.advance();
        Ok(op)
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.expect(&Token::KEY_RETURN)?;

        if !self.matches(&Token::TERM) {
            Ok(Stmt::Return(Some(self.parse_expr()?)))
        } else {
            Ok(Stmt::Return(None))
        }
    }

    fn parse_var_decl_stmt(&mut self) -> Result<Stmt, ParserError> {
        let name: String;
        let t: Type;
        let init: Option<Expr>;
        let mutable: bool = if self.matches(&Token::KEY_MUT) {
            true
        } else {
            false
        };
        name = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => {
                return Err(ParserError::new(self, "expected identifier"));
            }
        };
        self.expect(&Token::COLON)?;
        t = self.parse_type()?;
        if self.matches(&Token::OP_EQ) {
            init = Some(self.parse_expr()?);
        } else {
            init = None;
        }
        Ok(Stmt::VarDeclare {
            mutable: mutable,
            name: name,
            kind: t,
            init: init,
        })
    }

    fn parse_break_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.expect(&Token::KEY_BREAK)?;
        Ok(Stmt::Break)
    }

    fn parse_continue_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.expect(&Token::KEY_CONTINUE)?;
        Ok(Stmt::Continue)
    }

    ////////////////////
    // ! NOT FINISHED //
    ////////////////////
    fn parse_if_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.expect(&Token::KEY_IF)?;
        let condition = self.parse_expr()?;
        self.expect(&Token::KEY_THEN)?;
        if self.check(&Token::TERM) {
            self.advance();
        }
        let body = self.parse_stmt_list()?;
        let mut elifs = vec![];
        while self.matches_any(&[Token::KEY_ELIF, Token::TERM]) {
            if self.previous() == Token::TERM && !self.check(&Token::KEY_ELIF) {}
            let branch_expr = self.parse_expr()?;
            self.expect(&Token::KEY_THEN)?;
            if self.check(&Token::TERM) {
                self.advance();
            }
            let branch_body = self.parse_stmt_list()?;
            elifs.push((branch_expr, branch_body));
        }
        let mut else_body = None;
        if self.matches(&Token::KEY_ELSE) {
            if self.check(&Token::TERM) {
                self.advance();
            }
            else_body = Some(self.parse_stmt_list()?);
        }
        if self.check(&Token::TERM) {
            self.advance();
        }
        self.expect(&Token::KEY_END)?;
        Ok(Stmt::If {
            cond: condition,
            elif_branches: elifs,
            else_body: else_body,
            body: body,
        })
    }

    fn parse_switch_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.expect(&Token::KEY_SWITCH)?;
        let cond = self.parse_expr()?;
        self.expect(&Token::KEY_THEN)?;
        self.optional_term();
        let mut cases = vec![self.parse_switch_case()?];
        while self.check(&Token::KEY_CASE) {
            cases.push(self.parse_switch_case()?);
        }
        self.expect(&Token::KEY_END)?;
        Ok(Stmt::Switch {
            cond: cond,
            cases: cases,
        })
    }

    fn parse_switch_case(&mut self) -> Result<SwitchCase, ParserError> {
        if self.matches_any(&[Token::KEY_CASE, Token::KEY_DEFAULT]) {
            if self.previous() == Token::KEY_CASE {
                let case_cond = self.parse_expr()?;
                self.expect(&Token::COLON)?;
                self.optional_term();
                let case_body = self.parse_stmt_list()?;
                Ok(SwitchCase::Case {
                    pattern: case_cond,
                    body: case_body,
                })
            } else {
                self.expect(&Token::COLON)?;
                self.optional_term();
                let case_body = self.parse_stmt_list()?;
                Ok(SwitchCase::Default(case_body))
            }
        } else {
            Err(ParserError::new(
                self,
                "failed to parse switch case statment",
            ))
        }
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.expect(&Token::KEY_FOR)?;
        let index = match self.expect(&Token::IDENTIFER)?.to_string() {
            Some(s) => s,
            None => {
                return Err(ParserError::new(self, "failed to get index in for loop"));
            }
        };
        self.expect(&Token::KEY_IN)?;
        let iter = self.parse_expr()?;
        let mut iter_range = None;
        if self.matches(&Token::OP_RANGE) {
            iter_range = Some(self.parse_expr()?);
        }
        let body = self.parse_stmt_block()?;
        Ok(Stmt::For {
            ident: index,
            iter: if let Some(right) = iter_range {
                Iterator::Range {
                    min: iter,
                    max: right,
                }
            } else {
                Iterator::Object(iter)
            },
            body: body,
        })
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.expect(&Token::KEY_WHILE)?;
        let cond = self.parse_expr()?;
        let body = self.parse_stmt_block()?;
        Ok(Stmt::While {
            cond: cond,
            body: body,
        })
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParserError> {
        let mut list = vec![self.parse_param()?];
        while self.matches(&Token::COMMA) {
            list.push(self.parse_param()?);
        }
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
        if self.check(&Token::COLON) {
            self.advance();
        }
        #[cfg(debug_assertions)]
        println!("Parsing Base Type...");
        let base = self.parse_type_base()?;
        #[cfg(debug_assertions)]
        println!("Checking if type array...");
        if self.matches(&Token::LBRACKET) {
            if self.matches_any(&[Token::INT, Token::IDENTIFER]) {
                let size = self.parse_expr()?;
                self.expect(&Token::RBRACKET)?;
                Ok(Type::Array {
                    kind: base,
                    size: Box::new(size),
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
            lexer::Token::TypeI8 => TypePrim::I8,
            lexer::Token::TypeI16 => TypePrim::I16,
            lexer::Token::TypeI32 => TypePrim::I32,
            lexer::Token::TypeI64 => TypePrim::I64,
            lexer::Token::TypeU8 => TypePrim::U8,
            lexer::Token::TypeU16 => TypePrim::U16,
            lexer::Token::TypeU32 => TypePrim::U32,
            lexer::Token::TypeU64 => TypePrim::U64,
            lexer::Token::TypeF32 => TypePrim::F32,
            lexer::Token::TypeF64 => TypePrim::F64,
            lexer::Token::TypeChar => TypePrim::Char,
            lexer::Token::TypeBool => TypePrim::Bool,
            _ => {
                return Err(ParserError::new(self, "Invalid primary type"));
            }
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
                    #[cfg(debug_assertions)]
                    println!("parsing first arg in func/mem call");
                    let mut args = vec![self.parse_expr()?];
                    while self.matches(&Token::COMMA) {
                        #[cfg(debug_assertions)]
                        println!("parsing next arg in func/mem call");
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
            Token::Int(n) => {
                self.advance();
                Ok(Expr::Int(n))
            }
            Token::Float(n) => {
                self.advance();
                Ok(Expr::Float(n))
            }
            Token::True => {
                self.advance();
                Ok(Expr::True)
            }
            Token::False => {
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
            Token::Char(c) => {
                self.advance();
                Ok(Expr::Char(c))
            }
            Token::This => {
                self.advance();
                Ok(Expr::This)
            }
            Token::Identifier(s) => {
                if !(self.next() == Token::KEY_WITH) {
                    self.advance();

                    Ok(Expr::Ident(s))
                } else {
                    self.parse_struct_literal()
                }
            }
            Token::LPAREN => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RPAREN)?;
                Ok(expr)
            }
            _ => Err(ParserError::new(
                self,
                "unexpected token when parsing primary expression",
            )),
        }
    }

    // assign_block == struct_literal
    fn parse_struct_literal(&mut self) -> Result<Expr, ParserError> {
        #[cfg(debug_assertions)]
        println!("parse_struct_literal name: {:?}", self.previous());
        let name = self.peek();
        if self.next() == Token::TERM {
            self.advance();
            self.advance();
        } else {
            self.advance();
        }
        Ok(Expr::StructLiteral {
            name: name.to_string().unwrap(),
            fields: self.parse_assign_block()?,
        })
    }

    fn parse_assign_block(&mut self) -> Result<Vec<StructFieldInit>, ParserError> {
        self.expect(&Token::With)?;
        if self.check(&Token::Term) {
            self.advance();
        }
        let body = self.parse_field_list()?;
        if self.check(&Token::Term) {
            self.advance();
        }
        self.expect(&Token::End)?;
        Ok(body)
    }

    fn parse_field_list(&mut self) -> Result<Vec<StructFieldInit>, ParserError> {
        let mut list = vec![self.parse_field_init()?];
        while self.matches(&Token::TERM) {
            if self.check(&Token::KEY_END) {
                break;
            }
            list.push(self.parse_field_init()?);
        }
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
    pub fn peek(&self) -> lexer::Token {
        if self.pos < self.tokens.len() {
            self.tokens.get(self.pos).unwrap().clone()
        } else {
            Token::EOF
        }
    }

    pub fn previous(&self) -> lexer::Token {
        if self.pos == 0 {
            Token::EOF
        } else if (self.pos - 1) < self.tokens.len() {
            self.tokens.get(self.pos - 1).unwrap().clone()
        } else {
            Token::EOF
        }
    }

    pub fn next(&self) -> lexer::Token {
        if (self.pos + 1) < self.tokens.len() {
            self.tokens.get(self.pos + 1).unwrap().clone()
        } else {
            Token::EOF
        }
    }

    fn check(&self, tok: &lexer::Token) -> bool {
        if std::mem::discriminant(tok) == std::mem::discriminant(&self.peek()) {
            println!("({} == {}) TRUE", tok, &self.peek());
            true
        } else {
            println!("({} == {}) FALSE", tok, &self.peek());
            false
        }
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

    fn expect(&mut self, tok: &lexer::Token) -> Result<lexer::Token, ParserError> {
        let t = self.peek();
        #[cfg(debug_assertions)]
        println!("Expecting type {:?} Got {:?}", tok, t);

        if std::mem::discriminant(tok) != std::mem::discriminant(&t) {
            return Err(ParserError::new(self, format!("expected token: {}", *tok)));
        }
        #[cfg(debug_assertions)]
        println!("Adancing Pos");
        self.advance();
        Ok(t)
    }

    fn is_assign_op(&self) -> bool {
        matches!(
            self.peek(),
            Token::OP_EQ
                | Token::OP_PLUSEQ
                | Token::OP_SUBEQ
                | Token::OP_STAREQ
                | Token::OP_DIVEQ
                | Token::OP_MODEQ
                | Token::OP_AMPEQ
                | Token::OP_PIPEEQ
                | Token::OP_CARETEQ,
        )
    }

    fn optional_term(&mut self) {
        if self.check(&Token::TERM) {
            self.advance();
        }
    }

    pub fn span_object(&self) -> Span {
        if self.pos < self.span_table.len() {
            self.span_table[self.pos].clone()
        } else {
            Span {
                start: 0, // Byte offset
                end: 0,   // Byte offset Exclusive
                line: 0,
                col: 0,
            }
        }
    }
}
