# Semantic Analysis Implementation Plan                                    

## Context

 The src/symantic/ module has a skeleton (Symbol, SymbolTable, ScopeStack) but almost nothing works — only partial
 enum collection exists, and the code has several bugs that prevent compilation. This plan implements full semantic
  analysis: declaration collection, type/signature resolution, and body type-checking with fail-on-first-error
 semantics.

 Files to Modify

 - src/symantic/symbol.rs — fix derives, restructure SymbolKind with data
 - src/symantic/scope.rs — fix enter_scope/insert, add lookup
 - src/symantic/error.rs — add error kinds, improve Display
 - src/symantic/core.rs — implement all 3 passes (~500 lines)
 - src/main.rs — wire semantic analysis into the pipeline

 No new files needed.

 ---
## Step 1: Bug Fixes & symbol.rs Restructure

 Fix SymbolId — add #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

 Fix SymbolKind — rename Namspace → Namespace, enrich variants with data:
 pub enum SymbolKind {
     Const,
     Enum { variants: Vec<String> },
     Function { params: Vec<(String, Type)>, ret_type: Option<Type> },
     Namespace { scope_index: usize },
     Struct { fields: Vec<(String, Type)> },
     Type,
     Variable { mutable: bool },
 }

 Simplify Symbol — remove args: Vec<SymbolId> and value: Option<SymbolId> (these are now in SymbolKind). Remove
 set_args/set_value builders. Add get_mut to SymbolTable.

## Step 2: Fix scope.rs

 Fix enter_scope — set parent to current pos (currently pushes parent: None):
 pub fn enter_scope(&mut self) {
     let parent = if self.stack.is_empty() { None } else { Some(self.pos) };
     self.pos = self.stack.len();
     self.stack.push(Scope::new().set_parent_opt(parent));
 }

 Fix insert — take (name: String, sym: SymbolId), remove todo!(), return Result for duplicate detection.

 Add lookup — walk parent chain to find a symbol by name:
 pub fn lookup(&self, name: &str) -> Option<SymbolId> {
     let mut idx = self.pos;
     loop {
         if let Some(id) = self.stack[idx].symbols.get(name) { return Some(*id); }
         match self.stack[idx].parent {
             Some(p) => idx = p,
             None => return None,
         }
     }
 }

## Step 3: Improve error.rs

 Add SymanticErrorKind enum for structured errors:
 DuplicateDeclaration, UndefinedType, UndefinedVariable, UndefinedFunction,
 TypeMismatch, InvalidBreak, InvalidContinue, InvalidReturn,
 NotCallable, NotIndexable, NotAStruct, NotAMember,
 ImmutableAssignment, InvalidLValue, ArityMismatch, ScopeUnderflow

 Update SymanticError to store kind and optional Span. Change constructor to not require &ScopeStack. Improve
 Display to show the kind.

## Step 4: Pass 1 — Declaration Collection (analyse_definitions)

 Walk self.ast.nodes and register every Decl into the symbol table + current scope:

 - Enum → SymbolKind::Enum { variants }, insert name into scope
 - Const → SymbolKind::Const, store kind as datatype, insert into scope
 - Struct → SymbolKind::Struct { fields: vec of (name, type) }, insert into scope
 - Function → SymbolKind::Function { params, ret_type }, insert into scope
 - Member → same as Function but inserted with qualified name "Parent::method"
 - Namespace → SymbolKind::Namespace { scope_index }, enter new child scope, recurse, exit scope

 Must clone self.ast before iterating to avoid borrow conflicts with &mut self.

## Step 5: Pass 2 — Signature & Type Resolution (analyse_signatures)

 Walk declarations again. For every type reference (params, return types, struct fields, const types), validate it
 resolves to a known type via resolve_type:

 - TypeBase::Prim(_) → always valid
 - TypeBase::Ident(name) → look up in scope, must be Struct/Enum/Type kind
 - TypeBase::Ptr(inner) / TypeBase::Ref(inner) → recurse
 - Type::Array { kind, .. } → resolve the kind base

 For Decl::Member, additionally validate that parent refers to a known struct.

 For namespace decls, switch scopes.pos to the stored scope_index before resolving, then restore.

## Step 6: Pass 3 — Body Type-Checking (analyse_types)

 Context tracking — add to SymanticAnalysis:

 - loop_depth: u32 — incremented in while/for, checked by break/continue
 - current_fn_ret_type: Option<Option<Type>> — None = not in fn, Some(None) = void fn, Some(Some(T)) = returns T

 check_stmt(stmt) handles each statement variant:

 - VarDeclare → resolve type, if init present check it matches, register variable in scope
 - Assign → type-check both sides, check lvalue is mutable, check type compatibility
 - Return → check we're in a function, match return type
 - Break/Continue → check loop_depth > 0
 - If → check condition is bool, enter scope for each branch body
 - While → check condition is bool, increment loop_depth, enter scope for body
 - For → enter scope, infer loop var type from iterator (range → integer type, array → element type), register loop
  var, check body
 - Switch → check case pattern types match condition type
 - Expr → call type_of_expr for side-effect checking

 type_of_expr(expr) -> Result<Type> — the core type inference:
 ┌──────────────────────────────────┬───────────────────────────────────────────────────────────────────┐
 │            Expression            │                            Result Type                            │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Int(_)                           │ i64                                                               │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Float(_)                         │ f64                                                               │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Char(_)                          │ char                                                              │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ String(_)                        │ *char (pointer to char)                                           │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ True/False                       │ bool                                                              │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Nil                              │ *u8 (null pointer)                                                │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Ident(name)                      │ look up in scope, return datatype                                 │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ This                             │ look up "this" in scope                                           │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Arithmetic (Add/Sub/Mul/Div/Mod) │ check both numeric + compatible, return left type                 │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Comparison (Cmp/Gt/Lt/...)       │ check compatible, return bool                                     │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Logic (LogicOr/LogicAnd)         │ check both bool, return bool                                      │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Bitwise (BitwiseOr/Shift*)       │ check both integer, return left type                              │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Neg                              │ check numeric, return same                                        │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Not                              │ check bool, return bool                                           │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Tilde                            │ check integer, return same                                        │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Deref                            │ check pointer/ref, return inner type                              │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Cast { kind, .. }                │ resolve target type, return it                                    │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Call { func, args }              │ resolve callable → get params + ret_type, check arity + arg types │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ FuncCall { name, params }        │ look up by name, same as Call                                     │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Member { object, name }          │ get object type, find struct, find field                          │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Index { array, index }           │ check index is integer, array is Array/Ptr, return element type   │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ StructLiteral { name, fields }   │ look up struct, check all fields present + types match            │
 ├──────────────────────────────────┼───────────────────────────────────────────────────────────────────┤
 │ Ternary { a, b, c }              │ check a is bool, check b/c compatible, return b type              │
 └──────────────────────────────────┴───────────────────────────────────────────────────────────────────┘
 Helper functions needed:

 - resolve_callable(expr) -> (params, ret_type) — resolves Ident or Member to function info
 - resolve_member_type(obj_type, field) -> Type — looks up struct field type
 - extract_struct_name(type) -> String — unwraps Ident/Ptr/Ref to get struct name
 - check_types_compatible(expected, actual) — uses == on Type (has PartialEq)
 - check_numeric_type(t) / check_integer_type(t) / check_bool_type(t) — type classification
 - check_lvalue_mutable(expr) — walks expr to find the variable, checks Variable { mutable: true }
 - get_iterable_element_type(t) — extracts element type from Array

 Function/method body setup:

 - Enter new scope
 - Register params as Variable { mutable: false }
 - For methods: inject implicit this as &ParentStruct
 - Set current_fn_ret_type
 - Check all statements
 - Exit scope, restore context

## Step 7: Wire into main.rs

 After parsing succeeds (line 73), add:
 let mut analyser = symantic::SymanticAnalysis::new(ast.clone());
 if let Err(e) = analyser.analyse() {
     eprintln!("{}: Semantic Error: {}", filename, e);
     return ExitCode::FAILURE;
 }

## Verification

 1. cargo build — must compile cleanly
 2. cargo run -- examples/syntax.sl — should run through all 3 passes without panicking
 3. Test error cases manually:
   - Undefined variable reference → "undefined variable" error
   - Type mismatch in assignment → "type mismatch" error
   - break outside loop → "break outside of loop" error
   - Duplicate declaration → "duplicate declaration" error








# Semantic Analysis Implementation Chat (02/08/2026)

Let me check your project files for more context on your types.

I only have the two markdown files, not the actual Rust source. Let me work with what the user has shared and the grammar spec to help them implement the semantic analysis.

Key things I can see from their code:

    Namespace has a nodes field (Vec<Decl> presumably)
    Decl variants: Enum, Const, Struct, Member, Function, Namespace
    They have SymbolTable, ScopeStack, Symbol, SymbolId, SymbolKind
    They need help filling in the passes

Let me help them implement Pass 1 (collect definitions) and sketch out the architecture for the remaining passes. I need to make reasonable assumptions about the symbol table and scope stack APIs since I don't have those files.
Done

I only have the grammar/bytecode specs, not your full Rust source. But based on your code and grammar, I can see the shape of things. Here's where you're likely stuck and how to move forward:

Pass 1 (analyse_definitions) — register top-level names into the symbol table so later passes can resolve references. You're not resolving types yet, just claiming names and detecting duplicates.

Here's a concrete implementation assuming reasonable APIs on your SymbolTable/ScopeStack:
rust

fn analyse_definitions(&mut self) -> Result<(), SymanticError> {
    self.scopes.push_scope(); // global scope

    for decl in self.ast.nodes.iter() {
        match decl {
            Decl::Struct { name, fields } => {
                let id = self.symbols.insert(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Struct {
                        fields: fields.clone(),
                    },
                });
                self.scopes.define(name, id).map_err(|_| {
                    SymanticError::DuplicateSymbol(name.clone())
                })?;
            }

            Decl::Enum { name, variants } => {
                let id = self.symbols.insert(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Enum {
                        variants: variants.clone(),
                    },
                });
                self.scopes.define(name, id).map_err(|_| {
                    SymanticError::DuplicateSymbol(name.clone())
                })?;
            }

            Decl::Const { name, kind, value } => {
                let id = self.symbols.insert(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Const {
                        ty: kind.clone(),
                        value: value.clone(),
                    },
                });
                self.scopes.define(name, id).map_err(|_| {
                    SymanticError::DuplicateSymbol(name.clone())
                })?;
            }

            Decl::Function { name, params, ret_type, body } => {
                let id = self.symbols.insert(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Function {
                        params: params.clone(),
                        ret_type: ret_type.clone(),
                    },
                });
                self.scopes.define(name, id).map_err(|_| {
                    SymanticError::DuplicateSymbol(name.clone())
                })?;
                // Don't walk the body yet — that's pass 3+
            }

            Decl::Member { name, parent, params, ret_type, body } => {
                // Key: method names are scoped to their parent type.
                // You can use a mangled key like "Parent::method"
                // or store in a separate method table on the struct symbol.
                let qualified = format!("{}::{}", parent, name);
                let id = self.symbols.insert(Symbol {
                    name: qualified.clone(),
                    kind: SymbolKind::Method {
                        parent: parent.clone(),
                        params: params.clone(),
                        ret_type: ret_type.clone(),
                    },
                });
                self.scopes.define(&qualified, id).map_err(|_| {
                    SymanticError::DuplicateSymbol(qualified)
                })?;
            }

            Decl::Namespace(ns) => {
                // Recurse: push a new scope, register the namespace's decls
                self.scopes.push_scope();
                // You'll need a helper or make this method take a &[Decl]
                // so you can recurse into ns.nodes here
                self.analyse_definitions_inner(&ns.nodes)?;
                self.scopes.pop_scope();
            }
        }
    }

    Ok(())
}

The architectural issue — your analyse_definitions currently hardcodes self.ast.nodes. To handle Namespace recursion, extract the loop body into a helper:
rust

fn analyse_definitions(&mut self) -> Result<(), SymanticError> {
    self.scopes.push_scope();
    self.analyse_definitions_inner(&self.ast.nodes.clone())?;
    Ok(())
}

fn analyse_definitions_inner(&mut self, decls: &[Decl]) -> Result<(), SymanticError> {
    for decl in decls {
        match decl {
            // ... same arms as above ...
            Decl::Namespace(ns) => {
                let id = self.symbols.insert(Symbol {
                    name: ns.name.clone(),
                    kind: SymbolKind::Namespace,
                });
                self.scopes.define(&ns.name, id).map_err(|_| {
                    SymanticError::DuplicateSymbol(ns.name.clone())
                })?;
                self.scopes.push_scope();
                self.analyse_definitions_inner(&ns.nodes)?;
                self.scopes.pop_scope();
            }
        }
    }
    Ok(())
}

(Yes, the .clone() on self.ast.nodes is ugly — you'll likely want to refactor to take the AST by value or use indices to avoid the borrow conflict.)

For Pass 2 (analyse_signatures) — walk functions/methods again and resolve param/return types against the symbol table:
rust

fn analyse_signatures(&mut self) -> Result<(), SymanticError> {
    for decl in self.ast.nodes.iter() {
        match decl {
            Decl::Function { params, ret_type, .. }
            | Decl::Member { params, ret_type, .. } => {
                for p in params {
                    self.resolve_type(&p.ty)?;
                }
                if let Some(rt) = ret_type {
                    self.resolve_type(rt)?;
                }
            }
            Decl::Struct { fields, .. } => {
                for f in fields {
                    self.resolve_type(&f.ty)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn resolve_type(&self, ty: &Type) -> Result<(), SymanticError> {
    match ty {
        // Primitives (i32, f64, bool, etc.) are always valid
        Type::Primitive(_) => Ok(()),
        // Named types must exist in the symbol table
        Type::Named(name) => {
            self.scopes.lookup(name).ok_or_else(|| {
                SymanticError::UnknownType(name.clone())
            })?;
            Ok(())
        }
        Type::Pointer(inner) | Type::Ref(inner) => self.resolve_type(inner),
        Type::Array(inner, _) => self.resolve_type(inner),
    }
}

For Pass 3 (analyse_types / type-check bodies) — this is where you walk statement/expression ASTs inside function bodies, pushing/popping scopes for blocks, and inferring + checking expression types. This is the biggest pass. The core pattern is a recursive check_stmt / infer_expr pair.

What I'd suggest you focus on next:

    Nail down your SymbolKind enum and ScopeStack API (especially define returning Err on duplicates, and lookup returning Option<SymbolId>)
    Get Pass 1 compiling and tested with a simple struct + function
    Implement resolve_type for Pass 2
    Then tackle body type-checking incrementally — start with return, assignments, and binary ops on primitives before touching structs/methods

What specifically are you stuck on — the borrow checker fight with self.ast vs self.symbols, the scope stack design, or the type-checking logic itself?

currently I have run into a problem where my symbols have a circular dependancy in my symbol struct because the type is of type symbolid but that is just a differnt symbol so there is no base that is not actually a value. also at this point I am not sure how I want to define the value

pub struct Symbol {
    pub name: String,
    pub span: Span,
    pub datatype: Type,
    pub kind: SymbolKind,
    pub args: Vec<SymbolId>,
    pub value: Option<SymbolId>,
}

10:31 PM

The user has a circular dependency problem in their symbol table design. Their Symbol struct uses SymbolId for types, args, and values, but types themselves are symbols, creating infinite recursion with no base case. This is a classic compiler design issue.

The fix: types and values should NOT be SymbolId. Types need their own representation (a Type enum), and values need their own representation too. Only references to other named declarations should use SymbolId.
Done

The problem is you're conflating types with symbols. A type like i32 isn't a symbol — it's a property of a symbol. You need to separate the type representation from the symbol table.

Split Type into its own enum that bottoms out at primitives:
rust

/// Fully resolved type — no SymbolId for primitives
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Base cases — these break the cycle
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    F32, F64,
    Bool, Char,
    Void,                          // for functions with no return

    // References into the symbol table (structs, enums, aliases)
    Named(SymbolId),

    // Compound types built recursively
    Pointer(Box<Type>),
    Ref(Box<Type>),
    Array(Box<Type>, usize),
}

Primitives are leaf nodes — no SymbolId, no recursion. Named(SymbolId) is only for user-defined types like structs/enums that do live in the symbol table.

Then flatten Symbol — stop overloading it for everything:
rust

pub struct Symbol {
    pub name: String,
    pub span: Span,
    pub kind: SymbolKind,
}

pub enum SymbolKind {
    Variable {
        ty: Type,
        mutable: bool,
    },
    Const {
        ty: Type,
        // value is an expression from the AST, not a SymbolId
        value: Expr,
    },
    Function {
        params: Vec<(String, Type)>,
        ret: Type,
    },
    Method {
        parent: SymbolId,           // the struct it belongs to
        params: Vec<(String, Type)>,
        ret: Type,
    },
    Struct {
        fields: Vec<(String, Type)>,
    },
    Enum {
        variants: Vec<String>,
    },
    Namespace,
}
```

**Why your original design broke:**
```
Symbol.datatype -> Type(SymbolId) -> Symbol.datatype -> Type(SymbolId) -> ...
```

No base case. With the split, the chain is:
```
Symbol.kind.ty -> Type::I32                    // stops
Symbol.kind.ty -> Type::Named(id) -> Symbol    // one hop to a struct/enum definition

For value — don't store resolved values in the symbol table at all during semantic analysis. You have two options:

    Keep the AST Expr reference — Const { value: Expr } just points back at the parsed expression. During type-checking you verify the expr has the right type. During codegen you evaluate/emit it.
    Introduce a ConstValue enum if you want to fold constants early:

rust

pub enum ConstValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
}

I'd go with option 1 for now and defer constant folding to a later pass or codegen. Keeps the symbol table simple.

