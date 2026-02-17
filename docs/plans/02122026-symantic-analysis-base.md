First-Pass Semantic Analysis Plan

 Context

 The soluc/src/symantic/ module has a skeleton (Symbol, SymbolTable, ScopeStack) but almost nothing works — scope.rs has bugs that prevent correct scoping, symbol.rs is missing derives, and core.rs only has partial enum/const collection with duplicate-insertion bugs. The goal is to implement a working first
 pass that catches real bugs: duplicate declarations, undefined types, and structural errors like break outside a loop.

 Strategy: 3 passes, incrementally testable
 1. Declaration Collection — register all top-level names, detect duplicates
 2. Type Resolution — validate every type annotation refers to something real
 3. Structural Body Checking — scope variables, validate break/continue, walk control flow (defer full expression type-checking to a future iteration)

 ---
 Step 1: Fix symbol.rs — Derives & Typo

 File: soluc/src/symantic/symbol.rs

 - Add Copy and Hash to SymbolId derives (needed for HashMap<String, SymbolId> in scope)
 - Fix typo: Namspace(usize) → Namespace(usize) in SymbolKind
 - Add get_mut(&mut self, index: SymbolId) -> &mut Symbol to SymbolTable

 No structural changes — keep the existing Symbol { name, span, datatype, kind } shape and the parser's Type enum for datatype. A separate semantic type representation can come later when codegen needs it.

 Test: cargo build

 ---
 Step 2: Fix scope.rs — Parent Tracking, Insert, Lookup

 File: soluc/src/symantic/scope.rs

 Bug fixes:
 - ScopeStack::new() — initialize with a global scope (currently starts empty, which causes panics)
 - enter_scope() — set the new scope's parent to self.pos (currently pushes parent: None)
 - insert() — take (name: String, sym: SymbolId), return Result<(), SymanticError> for duplicate detection (currently inserts empty-string key with todo!())

 New methods:
 - lookup(&self, name: &str) -> Option<SymbolId> — walk parent chain to find a symbol
 - current_scope_index(&self) -> usize and set_scope(&mut self, idx: usize) — for namespace scope switching

 Test: cargo build

 ---
 Step 3: Simplify error.rs — Drop &ScopeStack Requirement

 File: soluc/src/symantic/error.rs

 - Change constructor from new(_ss: &ScopeStack, msg) to new(msg) (the &ScopeStack was unused anyway)
 - Add optional Span field for source location reporting
 - Add with_span(msg, span) constructor
 - Update Display to include line:col when span is present
 - Update scope.rs call sites to use new constructor

 Test: cargo build

 ---
 Step 4: Pass 1 — Declaration Collection (analyse_definitions)

 File: soluc/src/symantic/core.rs

 The core of the first pass. Walk the AST, register every declaration, detect duplicates.

 Architecture: Split into analyse_definitions() (clones self.ast.nodes to avoid borrow conflict) and collect_declarations(&mut self, decls: &[Decl]) (recursive helper for namespace nesting).
 ┌─────────────────────────────────────────┬─────────────────────────────────────────────┬────────────────────────────────────────────────────────────────┐
 │              Decl variant               │            Symbol registered as             │                             Notes                              │
 ├─────────────────────────────────────────┼─────────────────────────────────────────────┼────────────────────────────────────────────────────────────────┤
 │ Enum { name, variants }                 │ SymbolKind::Enum(variants)                  │ Fix: current code passes Vec::new() instead of actual variants │
 ├─────────────────────────────────────────┼─────────────────────────────────────────────┼────────────────────────────────────────────────────────────────┤
 │ Const { name, kind, .. }                │ SymbolKind::Const                           │ Fix: current code inserts twice (duplicate bug)                │
 ├─────────────────────────────────────────┼─────────────────────────────────────────────┼────────────────────────────────────────────────────────────────┤
 │ Struct { name, fields }                 │ SymbolKind::Struct { fields }               │ Convert Vec<StructFieldDecl> → Vec<(String, Type)>             │
 ├─────────────────────────────────────────┼─────────────────────────────────────────────┼────────────────────────────────────────────────────────────────┤
 │ Function { name, params, ret_type, .. } │ SymbolKind::Function { params, ret_type }   │ Convert Vec<Param> → Vec<(String, Type)>                       │
 ├─────────────────────────────────────────┼─────────────────────────────────────────────┼────────────────────────────────────────────────────────────────┤
 │ Member { name, parent, .. }             │ Same as Function, key = "Parent::method"    │ Qualified name for method lookup                               │
 ├─────────────────────────────────────────┼─────────────────────────────────────────────┼────────────────────────────────────────────────────────────────┤
 │ Namespace(ns)                           │ SymbolKind::Namespace(scope_idx)            │ Enter new scope, recurse into ns.nodes, exit scope             │
 ├─────────────────────────────────────────┼─────────────────────────────────────────────┼────────────────────────────────────────────────────────────────┤
 │ NamespaceMember { namespace, decl }     │ Lookup namespace scope, register decl there │ Error if namespace not found                                   │
 └─────────────────────────────────────────┴─────────────────────────────────────────────┴────────────────────────────────────────────────────────────────┘
 Human collaboration opportunity: The collect_declarations function involves a design decision about how to handle the namespace scope switching pattern (save/restore current scope index). This is a good candidate for hands-on implementation.

 What this catches: Duplicate declarations within the same scope.

 Test: cargo run -- examples/syntax.sl — should not panic

 ---
 Step 5: Pass 2 — Type Resolution (analyse_signatures)

 File: soluc/src/symantic/core.rs

 Walk every type annotation and validate it resolves to a known declaration.

 Key function: resolve_type(&self, ty: &Type) -> Result<(), SymanticError>
 - TypeBase::Prim(_) → always valid
 - TypeBase::Enum → always valid
 - TypeBase::Ident(qn) → look up in scope; for qualified names like math::Vector, resolve namespace first, then look in its scope
 - TypeBase::Ptr(inner) / TypeBase::Ref(inner) → recurse
 - Type::Array { kind, .. } → resolve the kind

 Prerequisite: Add accessor methods to QualifiedName in parser/types.rs (segments(), full_path(), name()) since its inner Vec<String> is private.

 What this catches: References to undefined types (e.g., speed: math::Nonexistent).

 Test: Modify syntax.sl to reference a nonexistent type → should get error

 ---
 Step 6: Pass 3 — Structural Body Validation (analyse_types)

 File: soluc/src/symantic/core.rs

 A lightweight body walk — no full expression type inference yet. Focus on structural correctness.

 check_bodies(decls) — for each Function/Member:
 1. Enter new scope
 2. Register parameters as SymbolKind::Variable(false) (immutable)
 3. For methods: register this as &ParentStruct
 4. Call check_stmts(body, ret_type, loop_depth=0)
 5. Exit scope

 check_stmts(stmts, ret_type, loop_depth) handles:
 - VarDeclare → resolve type, register variable in scope
 - Break / Continue → error if loop_depth == 0
 - If → enter/exit scope for each branch body
 - While / For → enter scope, increment loop_depth, check body
 - Switch → enter/exit scope for each case body
 - For → register loop variable in scope (placeholder i64 type for now)
 - Assign / Expr → skip (deferred)
 - Stray Elif / Else → error (shouldn't appear as standalone statements)

 What this catches: break outside loop, duplicate variable names in same scope, undefined types in variable declarations, correct block scoping.

 What is deferred: Expression type inference (type_of_expr), assignment type compatibility, return type matching, function call validation. These form a natural "second iteration" once this foundation is solid.

 Test: Add break outside a loop in syntax.sl → should get error

 ---
 Step 7: Wire into main.rs

 File: soluc/src/main.rs

 After the parser succeeds (line 73), add:
 let mut analyser = symantic::SymanticAnalysis::new(ast.clone());
 if let Err(e) = analyser.analyse() {
     eprintln!("{}: {}", filename, e);
     return ExitCode::FAILURE;
 }

 Test: cargo run -- examples/syntax.sl → should print "Semantic analysis passed" (or similar)

 ---
 Files Modified (in order)

 1. soluc/src/parser/types.rs — add QualifiedName accessors
 2. soluc/src/symantic/symbol.rs — fix derives, typo, add get_mut
 3. soluc/src/symantic/scope.rs — fix bugs, add lookup/set_scope
 4. soluc/src/symantic/error.rs — simplify constructor
 5. soluc/src/symantic/core.rs — implement all 3 passes (~250-350 lines)
 6. soluc/src/main.rs — wire in

 No new files needed.

 ---
 Verification

 1. cargo build — must compile after each step
 2. cargo run -- examples/syntax.sl — should pass all 3 analysis passes
 3. Manual error tests:
   - Duplicate declaration → "duplicate declaration: 'X'"
   - Undefined type → "undefined type 'X'"
   - break outside loop → "'break' outside of loop"

---

# Claude plan File

---

# First-Pass Semantic Analysis Plan

## Context

The `soluc/src/symantic/` module has a skeleton (Symbol, SymbolTable, ScopeStack) but almost nothing works — `scope.rs` has bugs that prevent correct scoping, `symbol.rs` is missing derives, and `core.rs` only has partial enum/const collection with duplicate-insertion bugs. The goal is to implement a **working first pass** that catches real bugs: duplicate declarations, undefined types, and structural errors like `break` outside a loop.

**Strategy: 3 passes, incrementally testable**
1. **Declaration Collection** — register all top-level names, detect duplicates
2. **Type Resolution** — validate every type annotation refers to something real
3. **Structural Body Checking** — scope variables, validate break/continue, walk control flow (defer full expression type-checking to a future iteration)

---

## Step 1: Fix `symbol.rs` — Derives & Typo

**File:** `soluc/src/symantic/symbol.rs`

- Add `Copy` and `Hash` to `SymbolId` derives (needed for `HashMap<String, SymbolId>` in scope)
- Fix typo: `Namspace(usize)` → `Namespace(usize)` in `SymbolKind`
- Add `get_mut(&mut self, index: SymbolId) -> &mut Symbol` to `SymbolTable`

No structural changes — keep the existing `Symbol { name, span, datatype, kind }` shape and the parser's `Type` enum for `datatype`. A separate semantic type representation can come later when codegen needs it.

**Test:** `cargo build`

---

## Step 2: Fix `scope.rs` — Parent Tracking, Insert, Lookup

**File:** `soluc/src/symantic/scope.rs`

**Bug fixes:**
- `ScopeStack::new()` — initialize with a global scope (currently starts empty, which causes panics)
- `enter_scope()` — set the new scope's parent to `self.pos` (currently pushes `parent: None`)
- `insert()` — take `(name: String, sym: SymbolId)`, return `Result<(), SymanticError>` for duplicate detection (currently inserts empty-string key with `todo!()`)

**New methods:**
- `lookup(&self, name: &str) -> Option<SymbolId>` — walk parent chain to find a symbol
- `current_scope_index(&self) -> usize` and `set_scope(&mut self, idx: usize)` — for namespace scope switching

**Test:** `cargo build`

---

## Step 3: Simplify `error.rs` — Drop `&ScopeStack` Requirement

**File:** `soluc/src/symantic/error.rs`

- Change constructor from `new(_ss: &ScopeStack, msg)` to `new(msg)` (the `&ScopeStack` was unused anyway)
- Add optional `Span` field for source location reporting
- Add `with_span(msg, span)` constructor
- Update `Display` to include line:col when span is present
- Update `scope.rs` call sites to use new constructor

**Test:** `cargo build`

---

## Step 4: Pass 1 — Declaration Collection (`analyse_definitions`)

**File:** `soluc/src/symantic/core.rs`

The core of the first pass. Walk the AST, register every declaration, detect duplicates.

**Architecture:** Split into `analyse_definitions()` (clones `self.ast.nodes` to avoid borrow conflict) and `collect_declarations(&mut self, decls: &[Decl])` (recursive helper for namespace nesting).

| Decl variant | Symbol registered as | Notes |
|---|---|---|
| `Enum { name, variants }` | `SymbolKind::Enum(variants)` | Fix: current code passes `Vec::new()` instead of actual variants |
| `Const { name, kind, .. }` | `SymbolKind::Const` | Fix: current code inserts twice (duplicate bug) |
| `Struct { name, fields }` | `SymbolKind::Struct { fields }` | Convert `Vec<StructFieldDecl>` → `Vec<(String, Type)>` |
| `Function { name, params, ret_type, .. }` | `SymbolKind::Function { params, ret_type }` | Convert `Vec<Param>` → `Vec<(String, Type)>` |
| `Member { name, parent, .. }` | Same as Function, key = `"Parent::method"` | Qualified name for method lookup |
| `Namespace(ns)` | `SymbolKind::Namespace(scope_idx)` | Enter new scope, recurse into `ns.nodes`, exit scope |
| `NamespaceMember { namespace, decl }` | Lookup namespace scope, register decl there | Error if namespace not found |

**Human collaboration opportunity:** The `collect_declarations` function involves a design decision about how to handle the namespace scope switching pattern (save/restore current scope index). This is a good candidate for hands-on implementation.

**What this catches:** Duplicate declarations within the same scope.

**Test:** `cargo run -- examples/syntax.sl` — should not panic

---

## Step 5: Pass 2 — Type Resolution (`analyse_signatures`)

**File:** `soluc/src/symantic/core.rs`

Walk every type annotation and validate it resolves to a known declaration.

**Key function:** `resolve_type(&self, ty: &Type) -> Result<(), SymanticError>`
- `TypeBase::Prim(_)` → always valid
- `TypeBase::Enum` → always valid
- `TypeBase::Ident(qn)` → look up in scope; for qualified names like `math::Vector`, resolve namespace first, then look in its scope
- `TypeBase::Ptr(inner)` / `TypeBase::Ref(inner)` → recurse
- `Type::Array { kind, .. }` → resolve the kind

**Prerequisite:** Add accessor methods to `QualifiedName` in `parser/types.rs` (`segments()`, `full_path()`, `name()`) since its inner `Vec<String>` is private.

**What this catches:** References to undefined types (e.g., `speed: math::Nonexistent`).

**Test:** Modify `syntax.sl` to reference a nonexistent type → should get error

---

## Step 6: Pass 3 — Structural Body Validation (`analyse_types`)

**File:** `soluc/src/symantic/core.rs`

A lightweight body walk — **no full expression type inference yet**. Focus on structural correctness.

**`check_bodies(decls)`** — for each Function/Member:
1. Enter new scope
2. Register parameters as `SymbolKind::Variable(false)` (immutable)
3. For methods: register `this` as `&ParentStruct`
4. Call `check_stmts(body, ret_type, loop_depth=0)`
5. Exit scope

**`check_stmts(stmts, ret_type, loop_depth)`** handles:
- `VarDeclare` → resolve type, register variable in scope
- `Break` / `Continue` → error if `loop_depth == 0`
- `If` → enter/exit scope for each branch body
- `While` / `For` → enter scope, increment `loop_depth`, check body
- `Switch` → enter/exit scope for each case body
- `For` → register loop variable in scope (placeholder `i64` type for now)
- `Assign` / `Expr` → skip (deferred)
- Stray `Elif` / `Else` → error (shouldn't appear as standalone statements)

**What this catches:** `break` outside loop, duplicate variable names in same scope, undefined types in variable declarations, correct block scoping.

**What is deferred:** Expression type inference (`type_of_expr`), assignment type compatibility, return type matching, function call validation. These form a natural "second iteration" once this foundation is solid.

**Test:** Add `break` outside a loop in `syntax.sl` → should get error

---

## Step 7: Wire into `main.rs`

**File:** `soluc/src/main.rs`

After the parser succeeds (line 73), add:
```rust
let mut analyser = symantic::SymanticAnalysis::new(ast.clone());
if let Err(e) = analyser.analyse() {
    eprintln!("{}: {}", filename, e);
    return ExitCode::FAILURE;
}
```

**Test:** `cargo run -- examples/syntax.sl` → should print "Semantic analysis passed" (or similar)

---

## Files Modified (in order)

1. `soluc/src/parser/types.rs` — add `QualifiedName` accessors
2. `soluc/src/symantic/symbol.rs` — fix derives, typo, add `get_mut`
3. `soluc/src/symantic/scope.rs` — fix bugs, add `lookup`/`set_scope`
4. `soluc/src/symantic/error.rs` — simplify constructor
5. `soluc/src/symantic/core.rs` — implement all 3 passes (~250-350 lines)
6. `soluc/src/main.rs` — wire in

No new files needed.

---

## Verification

1. `cargo build` — must compile after each step
2. `cargo run -- examples/syntax.sl` — should pass all 3 analysis passes
3. Manual error tests:
   - Duplicate declaration → `"duplicate declaration: 'X'"`
   - Undefined type → `"undefined type 'X'"`
   - `break` outside loop → `"'break' outside of loop"`

