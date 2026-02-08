use crate::lexer::Span;
use crate::parser::types::Type;

pub struct SymbolId(u32);

pub struct Symbol {
    pub name: String,
    pub span: Span,
    pub datatype: Type,
    pub kind: SymbolKind,
    pub args: Vec<SymbolId>,
    pub value: Option<SymbolId>,
}

impl Symbol {
    pub fn new(name: impl Into<String>, span: Span, t: Type, kind: SymbolKind) -> Self {
        Self {
            name: name.into(),
            span,
            datatype: t,
            kind,
            args: vec![],
            value: None,
        }
    }

    pub fn set_args(mut self, args: Vec<SymbolId>) -> Self {
        self.args = args;
        self
    }

    pub fn set_value(mut self, value: SymbolId) -> Self {
        self.value = Some(value);
        self
    }
}

pub enum SymbolKind {
    Const,
    Enum,
    Function,
    Namspace,
    Struct,
    Type,
    Variable(bool), // mutable or not
}
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }
    pub fn insert(&mut self, val: Symbol) -> SymbolId {
        let id = SymbolId(self.symbols.len() as u32);
        self.symbols.push(val);
        id
    }

    pub fn get(&self, index: SymbolId) -> &Symbol {
        &self.symbols[index.0 as usize]
    }
}
