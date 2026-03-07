# What is Zink?

Zink is a fast, readable scripting language built in Rust. It's designed to be the language you wish you'd learned first — clean syntax, zero boilerplate, and instant feedback.

## Goals

- **Readable by default** — No semicolons, no type annotations, no ceremony. Code reads like pseudocode.
- **Fast to run** — Rust-powered interpreter with quick startup and execution.
- **Easy to learn** — A small, consistent language with no hidden complexity.
- **Fun to write** — String interpolation, `say` for printing, and `loop N times` for counted loops.

## A Taste of Zink

```zink
# Variables
let name = "World"
let count = 5

# Functions
fn greet(who) {
  say "Hello, {who}!"
}

# Loops
loop count times {
  greet(name)
}

# Arrays + higher-order functions
let nums = [1, 2, 3, 4, 5]
let doubled = map(nums, fn(x) { return x * 2 })
say "Doubled: {doubled}"
```

## Architecture

Zink follows a classic interpreter pipeline:

```
Source Code → Lexer → Tokens → Parser → AST → Interpreter → Output
```

| Component       | File              | Description                      |
|-----------------|-------------------|----------------------------------|
| Lexer           | `lexer.rs`        | Tokenizes source into tokens     |
| Parser          | `parser.rs`       | Recursive-descent AST builder    |
| Interpreter     | `interpreter.rs`  | Tree-walk evaluator + builtins   |
| CLI             | `main.rs`         | File runner, REPL, stdin pipe    |

## Who is Zink for?

- **Beginners** who want a gentle introduction to programming concepts
- **Educators** who need a simple language for teaching
- **Hobbyists** who enjoy exploring language design
- **Rustaceans** who want to see a complete interpreter implementation
