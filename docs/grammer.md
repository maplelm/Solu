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

## Notation and Conventions

### Grammar Formalism

EBNF (Extended Backus-Naur Form) is used to specify Dune grammar.

#### Expression Grammer

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
```


#### Statement Grammar

The program grammer at a very high level can be summarized by

##### Program

```ebnf
program ::= declarations*
declarations ::= struct_decl
                | enum_decl
                | const_decl
                | function_decl
                | method_decl
```

##### Structs

```ebnf
struct_decl ::= "struct" identifier "is" TERM
                field_decl*
                "end" TERM
field_decl ::= identifier ":" type TERM
```

```ebnf
struct_literal ::= identifier "with" TERM
                field_init*
                "end" TERM
field_init ::= identifier "=" expr TERM
```

##### Enums

```ebnf
enum_decl ::= "enum" identifier "is" TERM
                (identifier TERM)*
                "end" TERM
```

##### Consts

```ebnf
const_decl ::= "const" identifier ":" type "=" expr TERM
```

##### Params

```ebnf
param_list ::= param ("," param)*
param ::= identifier ":" type
```

##### Functions

```ebnf
function_decl ::= identifier "(" param_list? ")" (":" type)? "do" TERM
                stmt*
                "end" TERM
```

##### Methods

```ebnf
method_decl ::= identifier "::" identifier "(" param_list? ")" (":" type)? "do" TERM
                stmt*
                "end" TERM
```

##### Types

```ebnf
type ::= type_base "[" expr "]"
type_base ::= primitive_type
                | identifier
                | "*" base_type
                | "&" base_type
primitive_type ::= "i8" | "i16" | "i32" | "i64"
                | "u8" | "u16" | "u32" | "u64"
                | "f32" | "f64" | "char" | "bool"

```

##### Statments

```ebnf
stmt ::= variable_decl
                | assignment_stmt
                | expr_stmt
                | return_stmt
                | break_stmt
                | continue_stmt
                | if_stmt
                | while_stmt
                | for_stmt
                | switch_stmt
variable_decl ::= "mut"? identifier ":" type ("=" expr)? TERM
assignment_stmt ::= lvalue assign_op expr TERM
lvalue ::= "*"* identifier (lvalue_suffix)*
lvalue_suffix ::= "." identifier
                | "[" expr "]"
assign_op ::= "=" | "+=" | "-=" | "*="
                | "/=" | "%=" | "&=" | "|="
                | "^=" | "<<=" | ">>="

expr_stmt ::= expr TERM

return_stmt ::= "return" expr? TERM
break_stmt ::= "break" TERM
continue_stmt ::= "continue" TERM

if_stmt ::= "if" expr "then" TERM
                stmt*
                ("elif" expr "then" TERM stmt*)*
                ("else" TERM stmt*)?
                "end" TERM

while_stmt ::= "while" expr "do" TERM
                stmt*
                "end" TERM

for_stmt ::= "for" identifier "in" expr  (".." expr )? "do" TERM 
                stmt*
                "end" TERM

switch_stmt ::= "switch" expr "do" TERM
                ("case" expr ":" TERM stmt*)*
                ("default" ":" TERM stmt*)?
                "end" TERM
```

##### Lexical Tokens

```ebnf
TERM ::= "\n"
nil ::= "nil"
bool_literal ::= "true" | "false"
hex_literal ::= "0x" hex_digit+
binary_literal ::= "0b" ("0" | "1")+
hex_digit ::= "0"..."9" | "a"..."f" | "A"..."F"
comment ::= "--" ? any_char_except_newline ?
```

### Lexical vs Syntactic Rules

UpperCamelCase → syntactic rules

lower_snake_case → lexical tokens

"literal" → exact text

? ... ? → lexer-defined pattern (regex or equivalent)

### Whitespace and Comments

whitespace ::= (" " | "\t" | "\n" | "\r")+
comment    ::= "--" anything? "\n"


State explicitly:

Where whitespace is ignored

Whether newlines are significant

## Lexical Grammar

### Character Set

UTF-8 / ASCII subset

Case sensitivity rules

### Identifiers
identifier ::= letter (letter | digit | "_")*
letter     ::= "a"…"z" | "A"…"Z"
digit      ::= "0"…"9"


Constraints:

Reserved keywords cannot be identifiers

Unicode policy (if any)

### Keywords



if, else, while, for, return, let, fn

### Literals

Integer Literals
int_literal ::= digit+

String Literals
string_literal ::= '"' (escape | ? any_char_except_quote ?) '"'
escape ::= "\" ("n" | "t" | "\" | "\")

## Syntactic Grammar

### Program Structure

program ::= declaration*

### Declarations

declaration ::= function_decl
              | variable_decl

### Statements

statement ::= expression_stmt
            | return_stmt
            | if_stmt
            | block

block ::= "{" statement* "}"

### Expressions

Operator Precedence (Informal)
Level	Operators	Associativity
1	* /	left
2	+ -	left
3	== !=	left
4	=	right
Formal Expression Grammar
expression ::= assignment

assignment ::= identifier "=" assignment
             | equality

equality ::= comparison (("==" | "!=") comparison)*

comparison ::= term ((">" | ">=" | "<" | "<=") term)*

term ::= factor (("+" | "-") factor)*

factor ::= unary (("*" | "/") unary)*

unary ::= ("!" | "-") unary
        | primary

primary ::= int_literal
          | string_literal
          | identifier
          | "(" expression ")"

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

Grammar is backward-compatible within a minor version

Breaking grammar changes require a major version bump

## Known Limitations and Open Questions

Track unresolved issues to avoid accidental inconsistencies.

- Should function calls allow trailing commas?
- Are implicit semicolons allowed?

## Change Log
2026-01-26 — Initial grammar draft
2026-01-27 - Added statement grammar

