use crate::{
    lexer::Span,
    parser::types::{Param, StructFieldDecl, Type},
};
use std::fmt;

/*
* Most production compilers separate these concerns into:
* > Symbol Table (binding layer)
* > Type Table / Type Arena (type system graph)
* > AST annotated with SymbolIds
* > Type checking phase attaches TypeIds
*/

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy, Hash)]
pub struct SymbolId(u32);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Symbol {
    pub name: String,
    pub span: Span,
    pub class: SymbolClass,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | {}", self.name, self.class)
    }
}
impl Symbol {
    pub fn new(name: impl Into<String>, t: SymbolClass) -> Self {
        Self {
            name: name.into(),
            span: Span {
                start: 0,
                end: 0,
                line: 0,
                col: 0,
            },
            class: t,
        }
    }

    pub fn set_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SymbolClass {
    Const(Type),
    Enum(Vec<String>),
    Struct(Vec<StructFieldDecl>),
    Primitive(Type),
    Function(Vec<Param>, Option<Type>),
    Member(Vec<Param>, Option<Type>),
    Constructor(Vec<Param>, Option<Type>),
}

impl fmt::Display for SymbolClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(unused)]
        match self {
            Self::Const(t) => write!(f, "Const"),
            Self::Enum(v) => write!(f, "Enum"),
            Self::Struct(fields) => write!(f, "Struct"),
            Self::Primitive(t) => write!(f, "Primitive"),
            Self::Function(v, rt) => write!(f, "Function"),
            Self::Member(v, rt) => write!(f, "Member"),
            Self::Constructor(v, rt) => write!(f, "Constructor"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SymbolOp {
    Declaration,
    Assignment,
    Expr,
}

impl fmt::Display for SymbolOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Declaration => write!(f, "Declaration"),
            Self::Assignment => write!(f, "Assignment"),
            Self::Expr => write!(f, "Expression"),
        }
    }
}

pub struct SymbolTable(Vec<Symbol>);

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, each) in self.0.iter().enumerate() {
            let _ = write!(f, "\t{}: {}\n", i, each);
        }
        fmt::Result::Ok(())
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, val: Symbol) -> SymbolId {
        let id = SymbolId(self.0.len() as u32);
        self.0.push(val);
        id
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.iter().any(|x| x.name == name)
    }

    pub fn get(&self, index: SymbolId) -> Option<&Symbol> {
        self.0.get(index.0 as usize)
    }

    pub fn get_mut(&mut self, index: SymbolId) -> Option<&mut Symbol> {
        self.0.get_mut(index.0 as usize)
    }
}
