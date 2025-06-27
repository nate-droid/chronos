# C∀O (Kao) Language Grammar

> **EBNF Grammar Specification for C∀O (Categorical ∀xiomatic Ordinal)**

This document defines the complete Extended Backus-Naur Form (EBNF) grammar for the C∀O programming language, a stack-based concatenative language built on mathematical foundations.

## Notation

- `::=` - defined as
- `|` - alternative
- `()` - grouping
- `[]` - optional (zero or one)
- `{}` - repetition (zero or more)
- `+` - one or more
- `*` - zero or more
- `""` - terminal symbols
- `<>` - non-terminal symbols

## Complete Grammar

### Top-Level Structure

```ebnf
program ::= { statement }

statement ::= expression
            | word_definition
            | type_signature_declaration
            | type_definition
            | axiom_declaration
            | comment

comment ::= "(" comment_text ")"
comment_text ::= { any_character_except_unmatched_parens }
```

### Expressions

```ebnf
expression ::= { token }

token ::= literal
        | word
        | quotation
        | match_expression

literal ::= natural_number
          | boolean
          | unit_value
          | string_literal

natural_number ::= digit { digit }
digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

boolean ::= "true" | "false"

unit_value ::= "()"

string_literal ::= '"' { string_character } '"'
string_character ::= any_character_except_quote
                   | escape_sequence
escape_sequence ::= "\" ( "n" | "t" | "r" | "\" | '"' )
```

### Words and Identifiers

```ebnf
word ::= identifier | operator

identifier ::= ( letter | "_" ) { letter | digit | "_" | "-" | "?" | "!" }

operator ::= single_char_operator | multi_char_operator

single_char_operator ::= "+" | "-" | "*" | "/" | "%" 
                       | "<" | ">" | "=" | "!"
                       | "&" | "|" | "^" | "~"
                       | "@" | "#" | "$"

multi_char_operator ::= "::" | "<=" | ">=" | "<>" | "->"

letter ::= "a" | "b" | "c" | ... | "z" | "A" | "B" | "C" | ... | "Z"
```

### Quotations (Code as Data)

```ebnf
quotation ::= "[" { token } "]"
```

### Word Definitions

```ebnf
word_definition ::= ":" identifier { token } ";"
```

### Type System

```ebnf
type_signature_declaration ::= "::" identifier [ type_signature ] ";"

type_signature ::= "(" [ type_list ] "->" [ type_list ] ")"

type_list ::= type { type }

type ::= primitive_type
       | composite_type
       | generic_type
       | type_variable

primitive_type ::= "Unit" | "Bool" | "Nat" | "Quote" | "Ordinal"

composite_type ::= identifier "{" { field_definition } "}"

field_definition ::= identifier ":" type

generic_type ::= "Option" "<" type ">"
               | "Result" "<" type "," type ">"
               | "List" "<" type ">"

type_variable ::= lowercase_identifier
```

### Type Definitions

```ebnf
type_definition ::= "type" identifier "{" { field_definition } "}"
```

### Axiom Declarations

```ebnf
axiom_declaration ::= "axiom" identifier type_signature ";"
```

### Pattern Matching

```ebnf
match_expression ::= "match" token { match_arm }

match_arm ::= "|" pattern "->" { token }

pattern ::= wildcard_pattern
          | variable_pattern
          | literal_pattern
          | constructor_pattern
          | list_pattern

wildcard_pattern ::= "_"

variable_pattern ::= identifier

literal_pattern ::= literal

constructor_pattern ::= identifier [ "(" pattern_list ")" ]

list_pattern ::= "[" [ pattern_list ] "]"

pattern_list ::= pattern { "," pattern }
```

## Lexical Rules

### Whitespace and Comments

```ebnf
whitespace ::= " " | "\t" | "\n" | "\r"

(* Comments are nested and can contain any text except unmatched parentheses *)
nested_comment ::= "(" { comment_content | nested_comment } ")"
comment_content ::= any_character_except_parens
```

### Character Classes

```ebnf
lowercase_identifier ::= lowercase_letter { letter | digit | "_" | "-" }
lowercase_letter ::= "a" | "b" | ... | "z"

any_character_except_quote ::= all_unicode_except ( '"' | "\" )
any_character_except_parens ::= all_unicode_except ( "(" | ")" )
```

## Operator Precedence and Associativity

C∀O uses postfix notation, so traditional operator precedence doesn't apply. All operations are performed left-to-right on the stack:

```cao
3 4 +        # 3 + 4 = 7
2 3 4 + *    # 2 * (3 + 4) = 14
```

## Language Examples

### Basic Expressions
```cao
3 4 +                    # Simple arithmetic
5 dup *                  # Square a number (5²)
true false and           # Boolean operations
[ 1 2 3 ] length        # List operations
```

### Word Definitions
```cao
: square dup * ;         # Define square function
: double 2 * ;           # Define double function
: max over over > [ drop ] [ swap drop ] if ;  # Maximum function
```

### Type Signatures
```cao
:: square ( Nat -> Nat ) ;
:: + ( Nat Nat -> Nat ) ;
:: dup ( a -> a a ) ;
:: if ( Bool Quote Quote -> ) ;
```

### Type Definitions
```cao
type Point {
    x: Nat
    y: Nat
}

type Maybe {
    some: Option<a>
    none: Unit
}
```

### Axiom Declarations
```cao
axiom termination_proof ( Ordinal -> Bool ) ;
axiom category_morphism ( a -> b ) ;
```

### Pattern Matching
```cao
match value
| Some(x) -> x 1 +
| None -> 0
```

## Stack Effect Notation

Type signatures use stack effect notation showing what types are consumed from and produced to the stack:

- `( -> Nat )` - produces a natural number (e.g., literals)
- `( Nat -> Nat )` - consumes and produces a natural number
- `( Nat Nat -> Nat )` - consumes two natural numbers, produces one
- `( a -> a a )` - duplicates any type (polymorphic)
- `( a b -> b a )` - swaps two values of any types

## Reserved Words

The following are reserved words in C∀O:

- **Type keywords**: `type`, `axiom`
- **Type signatures**: `::`
- **Definitions**: `:`
- **Primitive types**: `Unit`, `Bool`, `Nat`, `Quote`, `Ordinal`
- **Generic types**: `Option`, `Result`, `List`
- **Literals**: `true`, `false`
- **Pattern matching**: `match`
- **Control flow**: `if`, `while`, `loop`
- **Stack operations**: `dup`, `drop`, `swap`, `over`, `rot`

## Grammar Properties

1. **Concatenative**: No operator precedence; left-to-right evaluation
2. **Homoiconic**: Code and data have the same representation (quotations)
3. **Strongly typed**: Every expression has a well-defined type
4. **Stack-based**: All operations work on an implicit stack
5. **Mathematical**: Built on category theory and ordinal analysis

## Implementation Notes

- The lexer handles nested comments automatically
- Type inference can often eliminate the need for explicit type signatures
- Quotations are first-class values and can be manipulated like data
- All programs are guaranteed to terminate (provable via ordinal analysis)
- The grammar supports both interactive REPL use and file-based programs

---

*This grammar specification is part of the C∀O language documentation. For implementation details, see the parser module in the source code.*