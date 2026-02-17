#![allow(unused)]
use super::scope::ScopeStack;
use super::symbol::{Symbol, SymbolClass, SymbolId, SymbolTable};
use crate::lexer::Span;
use crate::parser::Namespace;
use crate::parser::types::{Decl, Type, TypeBase};
use crate::symantic::SymanticError;
use std::fmt::Write;

/*
* X Pass 1: Collect all top-level declarations
* X Pass 2: Parse namespaces recursivly
* Pass 3: Resolve Generics / Interfaces (Not going to implement in the first version)
* Pass 4: Resolve types and validate signatures
* Pass 5: Type-check function bodies
*/

pub struct NamespaceStack {
    active: String,
    history: Vec<String>,
}

impl NamespaceStack {
    pub fn new(start_point: impl Into<String>) -> Self {
        Self {
            active: start_point.into(),
            history: vec![],
        }
    }

    pub fn push(&mut self, ns: impl Into<String>) {
        if self.active.len() > 0 {
            self.history.push(self.active.clone());
        }
        self.active = ns.into();
    }

    pub fn pop(&mut self) -> String {
        self.active = self.history.pop().unwrap_or(String::new());
        self.active.clone()
    }

    pub fn peek(&self) -> &str {
        &self.active
    }

    pub fn prefix(&self, s: &str) -> String {
        let mut ns = String::new();
        for each in self.history.iter() {
            write!(ns, "{}::", each);
        }
        if self.active.len() > 0 {
            write!(ns, "{}::{}", self.active, s);
        } else {
            write!(ns, "{}", s);
        }
        ns
    }
}

pub struct SymanticAnalysis {
    pub symbols: SymbolTable,
    pub scopes: ScopeStack,
    pub ast: Namespace,
    pub ns_stack: NamespaceStack,
}

impl SymanticAnalysis {
    pub fn new(ast: Namespace) -> Self {
        Self {
            symbols: SymbolTable::new(),
            scopes: ScopeStack::new(),
            ns_stack: NamespaceStack::new(""),
            ast,
        }
    }

    pub fn analyse(&mut self) -> Result<(), super::error::SymanticError> {
        let ast_copy = self.ast.clone();
        self.analyse_definitions(&ast_copy)?;
        /*
            self.analyse_signatures()?;
            self.analyse_types()?;
        */
        println!("Symbols:\n{}", self.symbols);
        Ok(())
    }
}

////////////////////////
// Signature Analysis //
////////////////////////

impl SymanticAnalysis {
    fn analyse_signatures(&mut self) -> Result<(), super::error::SymanticError> {
        todo!()
    }

    fn valid_op(&mut self) -> Result<(), super::error::SymanticError> {
        todo!()
    }

    fn valid_fn_call(&self) -> Result<(), super::error::SymanticError> {
        todo!()
    }
}

///////////////////
// Type Analysis //
///////////////////

impl SymanticAnalysis {
    fn analyse_types(&mut self) -> Result<(), super::error::SymanticError> {
        todo!()
    }
}

/////////////////////////
// Definition Analysis //
/////////////////////////

impl SymanticAnalysis {
    fn analyse_definitions(&mut self, ns: &Namespace) -> Result<(), super::error::SymanticError> {
        for dec in ns.nodes.iter() {
            match dec {
                Decl::Enum { name, variants } => {
                    let full_name = self.ns_stack.prefix(name);
                    if self.symbols.contains(&full_name) {
                        return Err(SymanticError::new(
                            &self.scopes,
                            format!("Duplicate Declaration Found! {full_name}"),
                        ));
                    }
                    self.symbols
                        .insert(Symbol::new(full_name, SymbolClass::Enum(variants.clone())));
                }
                Decl::Const { name, kind, value } => {
                    let full_name = self.ns_stack.prefix(name);
                    if self.symbols.contains(&full_name) {
                        return Err(SymanticError::new(
                            &self.scopes,
                            format!("Duplicate Declaration Found! {full_name}"),
                        ));
                    }
                    self.symbols
                        .insert(Symbol::new(full_name, SymbolClass::Const(kind.clone())));
                }
                Decl::Struct { name, fields } => {
                    let full_name = self.ns_stack.prefix(name);
                    if self.symbols.contains(&full_name) {
                        return Err(SymanticError::new(
                            &self.scopes,
                            format!("Duplicate Declaration Found! {full_name}"),
                        ));
                    }
                    self.symbols
                        .insert(Symbol::new(full_name, SymbolClass::Struct(fields.clone())));
                }
                Decl::Member {
                    name,
                    parent,
                    params,
                    ret_type,
                    body,
                } => {
                    let full_name = format!("{}::{}", parent, name);
                    let full_name = self.ns_stack.prefix(&full_name);
                    if self.symbols.contains(&full_name) {
                        return Err(SymanticError::new(
                            &self.scopes,
                            format!("Duplicate Declaration Found! {full_name}"),
                        ));
                    }
                    let constructor = self.is_constructor(dec);
                    self.symbols.insert(Symbol::new(
                        full_name,
                        if constructor {
                            SymbolClass::Constructor(params.clone(), ret_type.clone())
                        } else {
                            SymbolClass::Member(params.clone(), ret_type.clone())
                        },
                    ));
                }
                Decl::Function {
                    name,
                    params,
                    ret_type,
                    body,
                } => {
                    let full_name = self.ns_stack.prefix(name);
                    if self.symbols.contains(&full_name) {
                        return Err(SymanticError::new(
                            &self.scopes,
                            format!("Duplicate Declaration Found! {name}"),
                        ));
                    }
                    self.symbols.insert(Symbol::new(
                        full_name,
                        SymbolClass::Function(params.clone(), ret_type.clone()),
                    ));
                }
                Decl::Namespace(n) => {
                    self.ns_stack.push(n.name.clone());
                    self.analyse_definitions(&n)?;
                    self.ns_stack.pop();
                }
            }
        }
        Ok(())
    }

    fn is_constructor(&mut self, mem: &Decl) -> bool {
        match mem {
            Decl::Member {
                name,
                parent,
                params,
                ret_type,
                body,
            } => {
                #[cfg(debug_assertions)]
                println!("is_construct: {} == {} ({})", name, parent, name == parent);
                name == parent
            }
            _ => {
                #[cfg(debug_assertions)]
                println!(
                    "is_construct: Decl Varient Passed Not a Member ({:?})!",
                    mem
                );
                false
            }
        }
    }
}

//////////////////////
// Helper Functions //
//////////////////////

impl SymanticAnalysis {
    fn insert(&mut self) {}
}
