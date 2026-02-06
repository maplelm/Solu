mod core;
mod error;
pub mod scope;
pub mod symbol;

pub use core::*;
pub use error::SymanticError;
pub use scope::{Scope, ScopeStack};
