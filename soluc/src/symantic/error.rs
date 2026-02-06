use super::scope::ScopeStack;
use std::fmt;

pub struct SymanticError {
    pub msg: String,
}

impl SymanticError {
    pub fn new(_ss: &ScopeStack, msg: impl Into<String>) -> Self {
        Self { msg: msg.into() }
    }
}

impl fmt::Display for SymanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symantic Error: {}", self.msg)
    }
}
