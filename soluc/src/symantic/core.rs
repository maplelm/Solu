#![allow(unused)]
use super::scope::ScopeStack;
use super::symbol::{Symbol, SymbolId, SymbolKind, SymbolTable};
use crate::parser::Namespace;
use crate::parser::types::Decl;

pub struct SymanticAnalysis {
    pub symbols: SymbolTable,
    pub scopes: ScopeStack,
    pub ast: Namespace,
}

impl SymanticAnalysis {
    pub fn new(ast: Namespace) -> Self {
        Self {
            symbols: SymbolTable::new(),
            scopes: ScopeStack::new(),
            ast,
        }
    }

    pub fn analyse(&mut self) {}

    fn analyse_definitions(&mut self) -> Result<(), super::error::SymanticError> {
        let mut pos = 0;
        for dec in self.ast.nodes.iter() {
            match dec {
                Decl::Enum { name, variants } => todo!(),
                Decl::Const { name, kind, value } => todo!(),
                Decl::Struct { name, fields } => todo!(),
                Decl::Member {
                    name,
                    parent,
                    params,
                    ret_type,
                    body,
                } => todo!(),
                Decl::Function {
                    name,
                    params,
                    ret_type,
                    body,
                } => todo!(),
                Decl::Namespace(n) => todo!(),
            }
        }
        Ok(())
    }

    fn insert(&mut self) {}
}
