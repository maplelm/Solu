# Dune Grammer Specification

## Table of contents

1. [Purpose and Scope](#purpose-and-scope)
2. [Notation and Conventions](#notation-and-conventions)
3. [Lexical Grammar](#lexical-grammar)
4. [Syntactic Grammar](#syntactic-grammar)
5. [Error Handling Rules](#error-handling-rules)
6. [Ambiguity and Disambiguation Rules](#ambiguity-and-disambiguation-rules)
7. [Grammar Stability Guarantees](#grammar-stability-guarantees)
8. [Known Limitations and Open Questions](#known-limitations-and-open-questions)
9. [Parser Mapping Notes (Optional but Recommended)](#parser-mapping-notes-optional-but-recommended)
10. [Change Log](#change-log)

## Purpose and Scope

Status: Draft / Stable / Experimental
Audience: Language implementer (parser, compiler, tooling)

This document defines the formal grammar of the language.
It is intended to:

Eliminate ambiguity in parsing

Serve as the single source of truth for syntax

Guide parser implementation and test generation

Non-goals:

Semantics (handled in semantics.md)

Runtime behavior

Standard library APIs

## Grammar & Syntax

### Full Grammar Spec

EBNF (Extended Backus-Naur Form) is used to specify Dune grammar.

__Level__ is of desending presedence. (1 == highest)

| Level |                Operands           |    Association    |
|-------|-----------------------------------|-------------------|
| 1     | Function calls / indexing         |         -         |
| 2     | Type casting ([type])             | Left-associative  |
| 3     | Unary (-, !, ~)                   | Right-associative |
| 4     | Multiplicative (*, /, %)          | Left-associative  |
| 5     | Additive (+, -)                   | Left-associative  |
| 6     | Comparison (==, !=, <, >, <=, >=) | Left-associative  |
| 7     | Bitwise Shift (<<, >>)            | Left-associative  |
| 8     | Bitwise AND ( & )                 | Left-associative  |
| 9     | Bitwise XOR ( ^ )                 | Left-associative  |
| 10    | Bitwise OR ( \| )                 | Left-associative  |
| 11    | Logical AND ( && )                | Left-associative  |
| 12    | Logical OR ( \|\| )               | Left-associative  |
| 13    | Ternary (? :)                     | Right-associative |

```ebnf
(* Starting Point *)
P ::= decl_list?

(* Declarations *)
decl_list ::= D (TERM D)*
D ::= struct_D  
    | enum_D 
    | const_D 
    | func_D
    | method_D
    | namespace_D
struct_D ::= "struct" identifier "is"
                TERM? member_list_D
                TERM? "end"
enum_D ::= "enum" identifier "is"
                TERM? variant_list
                TERM? "end"
const_D ::= "const" identifier ":" type "=" expr
func_D ::= identifier "(" params? ")" (":" type)? stmt_block
method_D ::= identifier "::" identifier "(" param_list? ")" (":" type)? stmt_block
namespace_D ::= "namespace" identifier "is TERM? decl_list TERM? "end"

member_list_D ::= member_D (TERM member_D)*
member_D ::= identifier ":" type
variant_list ::= variant (TERM variant)*
variant ::= identifier
params ::= param ("," param)*
param ::= identifier ":" type

(* Statements *)
stmt ::= var_D
         | assign_stmt
         | expr_stmt
         | ret_stmt
         | break_stmt
         | cont_stmt
         | if_stmt
         | while_stmt
         | for_stmt_
         | switch_stmt
var_D ::= "mut"? identifier ":" type ("=" expr)?
assign_stmt ::= lvalue assign_op (expr | assign_block)
expr_stmt ::= expr
ret_stmt ::= "return" expr?
break_stmt ::= "break"
cont_stmt ::= "continue"
if_stmt ::= "if" expr jmp_block
while_stmt ::= "while" expr stmt_block
for_stmt ::= "for" identifier "in" expr (".." expr)? stmt_block
switch_stmt ::= "switch" (expr | identifier) "then" TERM? 
                case_clause+
                default_clause?
                TERM? "end"

lvalue ::= "*"* identifier (lvalue_suffix)*
lvalue_suffix ::= "." identifier
                | "[" expr "]"
assign_op ::= "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^="
stmt_list ::= stmt (TERM stmt)*
assign_list := assign_stmt (TERM assign_stmt)*
case_clause ::= "case" expr ":"  TERM? stmt_list?
default_clause ::= "default" ":" TERM? stmt_list?

(* Blocks *)
jmp_block ::= "then" TERM? stmt_list
                (TERM? "elif" expr "then" TERM? stmt_list)*
                (TERM? "else" stmt_list)?
                TERM? "end"
stmt_block ::= "do" TERM? stmt_list
                TERM? "end"
decl_block ::= "is" TERM? decl_list
                TERM? "end"
assign_block ::= identifier "with" TERM? assign_list
                TERM? "end"

(* Types *)
type ::= type_base ("[" (identifier | digit+ ) "]")?
type_base ::= primitive_type
                | identifier
                | "*" type
                | "&" type
primitive_type ::= "i8" | "i16" | "i32" | "i64"
                | "u8" | "u16" | "u32" | "u64"
                | "f32" | "f64" | "char" | "bool"

(* Expressions *)
expr ::= ternary_expr
ternary_expr  ::= logical_or_expr ( "?" expr ":" ternary_expr)?
logical_or_expr ::= logical_and_expr ("||" logical_and_expr)*
logical_and_expr ::= bitwise_or_expr ("&&" bitwise_or_expr)*
bitwise_or_expr ::= bitwise_xor_expr ("|" bitwise_xor_expr)*
bitwise_xor_expr ::= bitwise_and_expr ("^" bitwise_and_expr)*
bitwise_and_expr ::= bitwise_shift_expr ("&" bitwise_shift_expr)*
bitwise_shift_expr ::= comparison_expr (("<<" | ">>")  comparison_expr)*
comparison_expr ::= additive_expr ( ("==" | "!=" | ">" | "<" | "<=" | ">=") additive_expr)*
additive_expr ::= multiplicative_expr (("+" | "-")  multiplicative_expr)*
multiplicative_expr ::= unary_expr ( ("*" | "/" | "%") unary_expr)*
unary_expr ::= ("-" | "!" | "~") unary_expr
                | type_cast_expr
                | postfix_expr
type_cast_expr ::= "[" type "]" unary_expr
primary_expr ::= int_literal
                | float_literal
                | bool_literal
                | string_literal
                | hex_literal
                | binary_literal
                | "nil"
                | struct_literal
                | identifier
                | "(" expr ")"
postfix_expr ::= primary_expr (postfix_op)*
postfix_op ::= "(" (expr ("," expr)*)? ")"
                | "." identifier
                | "[" expr "]"

(* Lexical Terms *)
TERM ::= "\n" | ";"
COMMENT ::= "--" ** "\n"
WHITESPACE ::= "\r" " " "\t" (("(" | "[" | "{")+"\n"(")" | "]" | "}"))+
NIL ::= "nil"
BOOL_LIT ::= "true" | "false"
HEX_LIT ::= "0x" hexdigit+
BIN_LIT ::= "0b" ("0" | "1")+
hex_digit ::= "0"..."9" | "a"..."f" | "A"..."F"
digit ::= ("0"..."9")
identifier ::= (letter | "_") (letter | digit | "_")*
letter     ::= "a"…"z" | "A"…"Z"
digit      ::= "0"…"9"
```

### Lexical vs Syntactic Rules

UpperCamelCase → syntactic rules

lower_snake_case → lexical tokens

"literal" → exact text

? ... ? → lexer-defined pattern (regex or equivalent)

### Whitespace and Comments

Newlines are Statement Terminators when not in cased with bracketing ( "()" |
"[]" | "{}" ). all other whitespace is ignored.

### Character Set

UTF-8 / ASCII subset

Case sensitivity rules

### Identifiers


Constraints:

Reserved keywords cannot be identifiers

Unicode policy (if any)

### Keywords

|        |       |          |        |       |
|--------|-------|----------|--------|-------|
| if     | then  | else     | elif   | while |
| for    | in    | do       | with   | is    |
| return | break | continue | switch | mut   |
| struct | enum  | const    | type   | arena |
| defer  | new   | true     | false  | nil   |
| i32    | i64   | u8       | u16    | u32   |
| char   | bool  | end      | i8     | i16   |
| u64    | f32   | f64      |        |       |

## Syntactic Grammar



## Error Handling Rules

Specify what is illegal, not just what is legal.

Missing semicolons

Invalid operator sequences

Unterminated blocks

Example:

It is a syntax error for `return` to appear outside a function body.

## Ambiguity and Disambiguation Rules

Document intentional decisions:

Dangling else resolution

Longest-match rules

Lookahead requirements

Example:

`else` binds to the nearest unmatched `if`.

## Grammar Stability Guarantees

- Grammar is backward-compatible within a minor version
- Breaking grammar changes require a major version bump

## Change Log
2026-01-26 — Initial grammar draft
2026-01-27 - Added statement grammar

