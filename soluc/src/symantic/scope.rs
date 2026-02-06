use super::error::SymanticError;
use crate::symantic::symbol::*;
use std::collections::HashMap;

pub struct Scope {
    pub parent: Option<usize>,
    pub symbols: HashMap<String, SymbolId>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn set_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }
}

pub struct ScopeStack {
    pub pos: usize,
    pub stack: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            pos: 0,
            stack: Vec::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.pos = self.stack.len();
        self.stack.push(Scope::new());
    }

    pub fn exit_scope(&mut self) -> Result<(), SymanticError> {
        match self.stack[self.pos].parent {
            Some(p) => self.pos = p,
            None => return Err(SymanticError::new(self, "Scope Underflow")),
        };
        Ok(())
    }

    pub fn insert(&mut self, sym: SymbolId) {
        self.stack[self.pos].symbols.insert("".to_string(), sym);
        todo!()
    }
}
