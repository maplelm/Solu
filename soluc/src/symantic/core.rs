#![allow(unused)]
use super::scope::ScopeStack;
use super::symbol::{Symbol, SymbolId, SymbolKind, SymbolTable};
use crate::parser::Namespace;
use crate::parser::types::Decl;

/*
* Pass 1: Collect all top-level declarations
* Pass 2: Parse namespaces recursivly
* Pass 3: Resolve Generics / Interfaces (Not going to implement in the first version)
* Pass 4: Resolve types and validate signatures
* Pass 5: Type-check function bodies
*/

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

    pub fn analyse(&mut self) -> Result<(), super::error::SymanticError> {
        self.analyse_definitions()?;
        self.analyse_signatures()?;
        self.analyse_types()?;
        Ok(())
    }

    fn analyse_signatures(&mut self) -> Result<(), super::error::SymanticError> {
        todo!()
    }

    fn analyse_types(&mut self) -> Result<(), super::error::SymanticError> {
        todo!()
    }

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
