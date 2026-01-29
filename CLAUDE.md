# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Solu is a systems programming language implementation written in Rust. It consists of two crates:
- **soluc** (package: `dvmc`): The compiler - lexer, parser, and AST generation
- **vm** (package: `DuneVM`): The virtual machine for bytecode execution

The project is early-stage (v0.1.0) with the lexer functional and parser partially implemented.

## Build Commands

```bash
# Build compiler
cd soluc && cargo build

# Build VM
cd vm && cargo build

# Run compiler on a source file
cd soluc && cargo run -- examples/syntax.sl

# Run compiler with token output
cd soluc && cargo run -- examples/syntax.sl --print
```

Source files use `.sl` extension (previously `.dn`).

## Architecture

```
SOURCE (.sl) → LEXER → TOKENS → PARSER → AST → [CODEGEN] → BYTECODE → VM
```

### Compiler (soluc/src/)

- **lexer/**: Tokenization
  - `core.rs`: Main `Lexer` struct with `tokenize()` entry point, handles numbers (int/float/hex/binary), strings, chars, operators, keywords
  - `token.rs`: `Token`, `Keyword`, `Operator`, `Delimeter` enums
  - `error.rs`: `LexerError` with span tracking
  - Comments: `--` style

- **parser/**: AST generation
  - `core.rs`: `Parser` struct with `parse()` entry, `Type` and `PrimitiveType` enums
  - `node.rs`: Top-level `Node` enum (Function, Struct, Enum, Const, TypeAlias)
  - `statements.rs`: `Stmt` enum and `Block` type
  - `expresions.rs`: Expression parsing (many functions still `todo!()`)
  - `ops.rs`: `BinOp`, `UnaryOp`, `AssignOp` definitions

### VM (vm/src/vm/)

- `bytecode.rs`: `Opcode` enum (24+ ops), `Instruction` struct (64-bit encoding), `Frame` for call stack, `Register` with type tags
- `cpu.rs`: `Vm` struct with bytecode execution, magic number `0x44564D31` ("DVM1")
- `memory.rs`: `Heap` and `Stack` structures
- `op_functions.rs`: Opcode implementations

## Language Syntax

Key patterns from `docs/grammar.md`:

```
-- Function
name(params) : type do
    ...
end

-- Method
Type::method(params) : type do
    ...
end

-- Struct
struct Name is
    field : type,
end

-- Control flow
if expr then ... elif expr then ... else ... end
while expr do ... end
for var in start..end do ... end
switch expr do case val then ... default ... end
```

Primitive types: `i8/16/32/64`, `u8/16/32/64`, `f32/64`, `char`, `bool`
Pointers: `*Type`, References: `&Type`, Arrays: `Type[size]`

## Current State

The lexer is functional but being refactored. Parser has AST node definitions complete but many parse functions are `todo!()`. Code generation is not yet implemented. VM has bytecode format defined but execution is incomplete.

Key incomplete areas:
- `NumberType` enum needs definition in lexer
- `Operand` and `VmState` types missing in VM
- Parser struct/enum/const parsing incomplete
- No code generation phase yet
