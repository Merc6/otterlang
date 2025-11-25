# OtterLang Language Specification

This document describes the syntax and semantics implemented by the current OtterLang compiler, runtime, LSP server, and standard library. Code samples use the fully supported `fn` syntax and reflect the features shipped in this repository.

## Table of Contents

1. [Lexical Structure](#lexical-structure)
2. [Type System](#type-system)
3. [Expressions](#expressions)
4. [Statements](#statements)
5. [Functions and Methods](#functions-and-methods)
6. [Structs](#structs)
7. [Enums](#enums)
8. [Pattern Matching](#pattern-matching)
9. [Modules and Visibility](#modules-and-visibility)
10. [Concurrency Primitives](#concurrency-primitives)
11. [Error Handling](#error-handling)
12. [Standard Library Overview](#standard-library-overview)
13. [Grammar Summary](#grammar-summary)
14. [Semantics and Implementation Notes](#semantics-and-implementation-notes)

## Lexical Structure

### Comments

OtterLang uses `#` for comments. Block comments are written as multiple single-line comments.

```otter
# This is a line comment
# Multiline comments are just repeated hash-prefixed lines
```

### Whitespace and Indentation

OtterLang is indentation-sensitive. A colon (`:`) ends phrase headers such as `fn`, `if`, `for`, `match case`, and `try`. A newline plus an increased indentation level introduces a block. When the indentation level decreases, the previous block ends. Tabs are not permitted; use spaces consistently (the formatter emits four spaces).

### Identifiers

Identifiers start with a letter or underscore and may contain ASCII letters, digits, or underscores. Unicode identifiers are also accepted. The standalone underscore (`_`) is treated as the wildcard identifier in patterns.

### Keywords

The lexer currently treats the following words as keywords and they cannot be used as user-defined identifiers:

```
fn, lambda, let, return, if, elif, else, for, while, break, continue, pass,
in, is, not, use, from, as, pub, async, await, spawn, match, case,
true, false, print, None, try, except, finally, raise, struct, enum,
and, or
```

`type` is a contextual keyword: it is recognized as the start of a type-alias declaration but otherwise behaves like an identifier.

### Literals

- **Numbers** support underscores for readability and may be written as integers (`42`, `1_000`) or floating-point values (`3.14`, `2.0e-3`).
- **Strings** use single or double quotes. Prefix a string with `f` to enable interpolation with `{expr}` placeholders.
- **Booleans** are `true` and `false`.
- **None/Unit** literals are written as `None`/`none` or as the empty tuple `()`.

## Type System

OtterLang uses a static type system with inference. Type annotations are optional but recommended for public APIs.

### Built-in Types

- `int`/`i64` – 64-bit signed integer
- `i32` – 32-bit signed integer
- `float`/`f64` – 64-bit floating point
- `bool` – boolean
- `str`/`string` – UTF-8 strings (both spellings are accepted throughout the standard library)
- `unit`/`None` – unit type; represents the absence of a value
- `list<T>` – homogenous dynamic array
- `dict<K, V>` – key/value map

Any identifier not in this list is treated as a type name. You can use that behavior to describe custom types (`User`, `Channel<string>`, `TaskHandle`) or to opt out of static checking with a conventional `any` type alias.

### Type Annotations

Annotate bindings, parameters, and return types with a colon:

```otter
let name: string = "Otter"
let values: list<int> = [1, 2, 3]
fn len_text(text: string) -> int:
    return len(text)
```

### Generics

Functions, structs, enums, and type aliases support generic parameters:

```otter
fn first<T>(items: list<T>) -> T:
    return items[0]

struct Pair<T, U>:
    first: T
    second: U
```

### Type Aliases

Define aliases with the contextual `type` keyword:

```otter
pub type UserId = int
pub type Response<T> = Result<T, Error>
```

## Expressions

### Arithmetic and Comparison

OtterLang supports `+`, `-`, `*`, `/`, and `%`. The `+` operator also performs string concatenation, automatically converting integers, floats, and booleans to strings. Comparison operators include `==`, `!=`, `<`, `>`, `<=`, `>=`, `is`, and `is not`.

```otter
let normalized = (value - min) / (max - min)
if count is not None and count > 0:
    print("ready")
```

### Logical Operators

Use `and`, `or`, and `not` for boolean logic.

```otter
if is_ready and not has_failed:
    proceed()
```

### Function and Method Calls

Call syntax uses parentheses. Methods are regular functions stored inside structs, so you call them with the dot operator: `point.distance()`.

### Member Access and Namespaces

Use `object.field` or `Module.symbol`. Enum variants use the same syntax: `Option.Some(value)`.

### Struct Instantiation

Structs use keyword-style arguments:

```otter
let origin = Point(x=0.0, y=0.0)
```

### Collection Literals

```otter
let numbers = [1, 2, 3]
let mapping = {"a": 1, "b": 2}
```

### Comprehensions

Lists and dictionaries support comprehension syntax with an optional `if` filter:

```otter
let squares = [x * x for x in 0..10]
let indexed = {x: idx for idx in 0..len(items) if items[idx] != None}
```

### Range Expressions

`start..end` produces a range expression. Ranges are evaluated eagerly inside `for` loops and are exclusive of `end`.

```otter
for i in 0..count:
    println(str(i))
```

### Lambda Expressions

Lambdas create anonymous functions. Bodies can be a single expression or an indented block.

```otter
let doubler = lambda (value: int) -> int: value * 2
let handler = lambda (event):
    if event.kind == "update":
        process(event)
```

### Await and Spawn

`await` consumes the result of an asynchronous computation. `spawn` starts an asynchronous computation and returns a task handle.

```otter
let task = spawn fetch_data(url)
let payload = await task
```

### F-Strings and Interpolation

Prefix strings with `f` to embed arbitrary expressions:

```otter
let summary = f"Processed {len(items)} items in {duration_ms}ms"
```

## Statements

### Variable Declarations and Assignment

Use `let` to introduce bindings. `pub let` exports a binding from the current module.

```otter
let total = 0.0
pub let version: string = runtime.version()
```

Simple reassignments omit `let`:

```otter
total = total + chunk
items += [extra]
```

### Expression Statements

Any expression can appear as a statement. This is how function calls and comprehensions that produce side effects are executed.

### Control Flow

#### `if` / `elif` / `else`

```otter
if size == 0:
    return
elif size < 10:
    print("small batch")
else:
    print("large batch")
```

#### `while`

```otter
while remaining > 0:
    remaining -= 1
```

#### `for`

`for` iterates over any iterable expression. Ranges are the easiest way to create numeric loops.

```otter
for user in users:
    println(user.name)
```

#### `match`

`match` dispatches on patterns. Guards (`case ... if ...`) are not supported in the current grammar.

```otter
let description = match result:
    case Result.Ok(value):
        f"ok: {value}"
    case Result.Err(error):
        f"error: {error}"
```

#### `try` / `except` / `else` / `finally`

```otter
try:
    risky_call()
except Error as err:
    log(err)
else:
    println("all good")
finally:
    cleanup()
```

#### `raise`

`raise` rethrows the current error when no argument is supplied, or raises the supplied expression.

### Loop Control

Use `break`, `continue`, and `pass` inside loops or placeholders. `return` exits the current function.

## Functions and Methods

Functions use the following syntax:

```otter
pub fn greet(name: string, greeting: string = "Hello") -> string:
    return f"{greeting}, {name}!"
```

- Parameters can have default values. Once a parameter declares a default, all subsequent parameters must also declare defaults.
- Functions may declare type parameters: `fn parse<T>(text: string) -> T`.
- Nested functions are allowed.
- Method definitions live inside `struct` blocks and take `self` explicitly as the first parameter.

Top-level code may only contain function definitions, `let` statements, struct/enum/type declarations, `use`/`pub use` statements, and expression statements. Other control-flow constructs must appear inside functions.

## Structs

Structs group named fields and optional methods.

```otter
pub struct Point:
    x: float
    y: float

    fn distance(self) -> float:
        return math.sqrt(self.x * self.x + self.y * self.y)
```

Instantiate structs with keyword arguments: `Point(x=3.0, y=4.0)`.

Struct definitions can declare generics: `struct Box<T>:`.

## Enums

Enums define tagged unions. Variants either carry payloads or act as unit variants.

```otter
pub enum Result<T, E>:
    Ok: (T)
    Err: (E)
```

Construct variants via `Result.Ok(value)`/`Result.Err(error)` and pattern match on them in `match` expressions.

## Pattern Matching

Patterns supported by the parser and type checker:

- `_` – wildcard
- `identifier` – binds the value
- Literals – match exact values
- Enum variants – `Option.Some(value)`
- Struct destructuring – `Point{x, y}` (optional nested patterns per field)
- Array/list patterns – `[head, ..rest]`

Patterns appear in `match case` headers and in destructuring `let` bindings inside match arms.

## Modules and Visibility

Each `.ot` file defines a module. Items are private by default. Mark functions, structs, enums, `let` bindings, and type aliases with `pub` to export them. The compiler supports two import forms:

```otter
use std/io as io
use math, std/time as time

pub use core.Option
pub use math.sqrt as square_root
```

Module paths consist of segments separated by `/` or `:` (`use std/io`) and may start with `.` or `..` for relative imports. Transparent Rust FFI uses the same mechanism (`use rust:serde/json`).

`pub use` statements re-export items or entire modules. `pub use math` re-exports everything that `math` already exposes as `pub`.

## Concurrency Primitives

OtterLang currently ships two levels of concurrency support:

1. **Language-level operators**: `spawn fn_call(...)` runs a function asynchronously and returns a task handle. `await handle` waits for completion and yields the underlying result.
2. **Standard library**: `stdlib/otter/task.ot` exposes helpers for spawning tasks, joining/detaching them, sleeping, working with typed channels, and building `select` statements.

Example:

```otter
let worker = spawn process_batch(batch)
let snapshot = spawn fetch_snapshot()
let batch_result = await worker
let snapshot_result = await snapshot
```

## Error Handling

- `try/except/else/finally` wraps statements, as shown earlier.
- `raise` rethrows errors or raises custom values.
- `panic(message)` is a built-in for unrecoverable failures.
- `Result<T, E>` and `Option<T>` live in `stdlib/otter/core.ot` and provide algebraic error handling.

## Standard Library Overview

The `stdlib/otter` directory contains the modules shipped with the compiler. Import them with `use` statements.

- **builtins** – fundamental helpers such as `len`, `cap`, list/map mutation, `panic`, `recover`, `type_of`, `append`, `range`, and structured error utilities (`try_func`, `select`, `defer`).
- **core** – definitions of `Option<T>` and `Result<T, E>`.
- **fmt** – lightweight wrappers around standard output (`print`, `println`, `eprintln`).
- **fs** – filesystem helpers: `exists`, `mkdir`, `remove`, `list_dir`, file IO shortcuts, etc.
- **http** – convenience wrappers for HTTP verbs built on the runtime networking stack.
- **io** – file IO plus buffered IO helpers.
- **json** – encoding/decoding JSON strings, pretty printing, and validation.
- **math** – numeric algorithms (`sqrt`, `pow`, `exp`, `clamp`, `randf`, etc.).
- **net** – TCP-style networking primitives plus HTTP response helpers.
- **rand** – RNG seeding plus integer/float random generators.
- **runtime** – introspection helpers (`gos`, `cpu_count`, `memory`, `stats`, `version`).
- **task** – task spawning, sleeping, typed channels, and select utilities.
- **time** – timestamps, sleeping, timers, formatting, and parsing.

Each module is pure OtterLang code and may be used as a reference for idiomatic syntax.

## Grammar Summary

The grammar below omits whitespace and indentation management for brevity.

```
program       := (use_stmt | pub_use_stmt | type_alias | struct_def | enum_def | function | let_stmt | statement)*
function      := [pub] fn identifier ["<" type_params ">"] "(" params ")" ["->" type] ":" block
params        := param ("," param)*
param         := identifier [":" type] ["=" expr]
struct_def    := [pub] struct identifier ["<" type_params ">"] ":" newline indent struct_body dedent
enum_def      := [pub] enum identifier ["<" type_params ">"] ":" newline indent enum_variant+ dedent
 type_alias   := [pub] type identifier ["<" type_params ">"] "=" type

statement     := let_stmt
               | assignment_stmt
               | augmented_assignment
               | return_stmt
               | break_stmt
               | continue_stmt
               | pass_stmt
               | if_stmt
               | while_stmt
               | for_stmt
               | try_stmt
               | match_expr
               | expr_stmt

expr          := lambda_expr
               | await_expr
               | spawn_expr
               | range_expr
               | comprehension
               | literal
               | identifier
               | struct_init
               | dict_literal
               | list_literal
               | call_expr
               | member_expr
               | unary_expr
               | binary_expr

pattern       := "_"
               | literal
               | identifier
               | enum_pattern
               | struct_pattern
               | list_pattern
```

## Semantics and Implementation Notes

- **Type Checking** – Static type checking with inference is performed before code generation. Generic parameters default to unconstrained type variables.
- **Evaluation Order** – Expressions evaluate left-to-right. Function arguments are evaluated before the call.
- **Memory Management** – The runtime manages memory automatically using reference counting and runtime support utilities in `runtime/`.
- **Code Generation** – The `otter` binary can target LLVM or Cranelift backends. Both backends eventually emit machine code to run programs natively.
- **Tooling** – The repository ships a formatter, language server, REPL, and VS Code syntax highlighter that all understand the syntax described in this document.

