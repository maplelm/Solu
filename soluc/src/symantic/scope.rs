use super::error::SymanticError;
use crate::symantic::symbol::*;
use std::collections::HashMap;

pub struct ScopePosition {
    pos: usize,
    history: Vec<usize>,
}

impl From<ScopePosition> for usize {
    fn from(value: ScopePosition) -> Self {
        value.pos
    }
}

impl ScopePosition {
    pub fn new(start_pos: usize) -> Self {
        Self {
            pos: start_pos,
            history: vec![],
        }
    }

    pub fn next(&mut self, index: impl Into<usize>) {
        self.history.push(self.pos);
        self.pos = index.into()
    }

    pub fn back(&mut self) -> usize {
        self.pos = self.history.pop().unwrap_or(0);
        self.pos
    }

    pub fn prev(&self) -> usize {
        match self.history.last() {
            Some(n) => n.clone(),
            None => 0,
        }
    }

    pub fn index(&self) -> usize {
        self.pos
    }

    pub fn depth(&self) -> usize {
        self.history.len()
    }
}

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

    pub fn set_parent(mut self, parent: impl Into<usize>) -> Self {
        self.parent = Some(parent.into());
        self
    }
}

pub struct ScopeStack {
    pub pos: ScopePosition,
    pub stack: Vec<Scope>,
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            pos: ScopePosition::new(0),
            stack: vec![Scope::new()],
        }
    }

    pub fn enter_scope(&mut self) -> usize {
        self.pos.next(self.stack.len());
        self.stack.push(Scope::new().set_parent(self.pos.prev()));
        self.pos.index()
    }

    pub fn exit_scope(&mut self) -> Result<usize, SymanticError> {
        if self.pos.depth() > 0 {
            Ok(self.pos.back())
        } else {
            Err(SymanticError::new(self, "Scope Underflow"))
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, sym: SymbolId) {
        self.stack[self.pos.index()].symbols.insert(key.into(), sym);
    }
}
