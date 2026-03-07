# Roadmap — Zink v0.1.0 → v1.0

> **"Say what you mean. Mean what you say."**

The path from a working prototype to a production-ready, stable language.

### Guiding Philosophy

Zink is a language for clarity, not cleverness. Every feature on this roadmap is evaluated against the [Zen of Zink](README.md#the-zen-of-zink) — 10 principles that prioritize readability, simplicity, and obviousness. If a feature makes the language harder to read, it doesn't ship. If there's already one way to do something, we don't add a second.

---

## Current State: v0.1.0 (March 2026)

**What exists today:**

- Tree-walk interpreter in Rust (lexer → parser → interpreter)
- 5 types: number (f64), string, boolean, null, array
- Control flow: `if`/`else`, `while`, `loop N times`
- Functions: first-class, closures, recursion, lambdas
- 28 built-in functions (math, strings, arrays, higher-order)
- String interpolation with `{expr}`
- CLI: file execution, REPL (multi-line), stdin pipe
- Browser playground (React + WASM)
- VitePress docs site
- VS Code extension (syntax, snippets, REPL)
- CI/CD: build, test, deploy, release (3 OS + VSIX)

**What's missing for v1.0:**

- No `for` loop or `for..in` iteration
- No `break` / `continue`
- No error handling (`try`/`catch`)
- No modules / imports
- No standard library beyond builtins
- No objects / maps / dictionaries
- No string methods via dot notation
- No compound assignment (`+=`, `-=`)
- No destructuring
- No test framework
- No package manager
- Tree-walk only (no bytecode, no compilation)
- No LSP (Language Server Protocol)
- No formatter / linter for Zink code
- No debugger integration
- Limited error messages (many show line 0)

---

## Phase 1 — Language Completeness (v0.2.0)

**Goal:** Make Zink a complete scripting language with all essential control flow and data structures.

### 1.1 — Control Flow

- [ ] **`for..in` loop** — iterate arrays: `for x in arr { say x }`
- [ ] **`for` range loop** — iterate ranges: `for i in range(0, 10) { say i }`
- [ ] **`break`** — exit loops early
- [ ] **`continue`** — skip to next iteration
- [ ] **Loop index in `loop N times`** — expose `it` or `_` as the iteration variable
- [ ] **Ternary / conditional expression** — `let x = if cond { a } else { b }` (expression-based if)

### 1.2 — Data Structures

- [ ] **Maps / Dictionaries** — `let m = { name: "Zink", version: 1 }`
- [ ] **Map access** — `m["key"]` and `m.key`
- [ ] **Map builtins** — `keys(m)`, `values(m)`, `has(m, key)`, `remove(m, key)`
- [ ] **Map iteration** — `for key in keys(m) { ... }`
- [ ] **Tuples** (optional) — `let pair = (1, 2)` — lightweight fixed-size grouping
- [ ] **Spread operator** — `let merged = [...arr1, ...arr2]`

### 1.3 — Operators

- [ ] **Compound assignment** — `+=`, `-=`, `*=`, `/=`, `%=`
- [ ] **Increment / decrement** — `x += 1` (no `++`/`--` to stay readable)
- [ ] **String repetition** — `"ha" * 3` → `"hahaha"`
- [ ] **`in` operator** — `if x in arr { ... }`, `if key in map { ... }`

### 1.4 — String & Array Enhancements

- [ ] **Multi-line strings** — triple-quote or backtick syntax
- [ ] **Raw strings** — `r"no \escapes"` or similar
- [ ] **String `.length`** — already works via `.length` property
- [ ] **Negative indexing** — `arr[-1]` returns last element
- [ ] **Array slicing syntax** — `arr[1:3]` as sugar for `slice(arr, 1, 3)`
- [ ] **Destructuring assignment** — `let [a, b] = [1, 2]`, `let { x, y } = point`

### 1.5 — Functions

- [ ] **Default parameters** — `fn greet(name, greeting = "Hello") { ... }`
- [ ] **Rest parameters** — `fn sum(...nums) { ... }`
- [ ] **Named arguments** (optional) — `greet(name: "Alice")`
- [ ] **Arrow syntax** (maybe) — `let double = (x) => x * 2`

---

## Phase 2 — Error Handling & Quality (v0.3.0)

**Goal:** Proper error handling, better error messages, and a test framework.

### 2.1 — Error Handling

- [ ] **`try` / `catch`** — structured error handling
  ```
  try {
    let x = num("not a number")
  } catch err {
    say "Error: {err}"
  }
  ```
- [ ] **`throw`** — user-defined errors: `throw "invalid input"`
- [ ] **Error values** — errors as first-class values with message and line
- [ ] **`finally`** (optional) — cleanup block

### 2.2 — Error Messages

- [ ] **Line numbers on all runtime errors** — currently many show `line 0`
- [ ] **Column numbers** — more precise error locations
- [ ] **Source snippets in errors** — show the offending line of code
- [ ] **Stack traces** — show call stack on runtime errors
- [ ] **Colored error output** — `[error]`, `[warning]` with ANSI colors in terminal

### 2.3 — Testing

- [ ] **`assert` builtin** — `assert(x == 42)`, `assert(x == 42, "x should be 42")`
- [ ] **`assert_eq` / `assert_ne`** — structured assertions
- [ ] **Test runner (built-in)** — `zink test tests/` or `zink --test file.zink`
- [ ] **Test blocks in language** — `test "addition" { assert_eq(1 + 1, 2) }`
- [ ] **Rust test suite** — comprehensive `#[test]` coverage for lexer, parser, interpreter
- [ ] **CI integration** — tests run on every PR

### 2.4 — Developer Experience

- [ ] **`typeof` as keyword or keep `type()` builtin** — decide and document
- [ ] **REPL improvements** — history, up/down arrow, tab completion, syntax highlighting
- [ ] **Better REPL multi-line** — detect incomplete expressions, not just braces
- [ ] **`--version` flag** — `zink --version` prints version
- [ ] **`--help` flag** — `zink --help` prints usage
- [ ] **Exit codes** — `0` for success, `1` for runtime error, `2` for parse error

---

## Phase 3 — Modules & Standard Library (v0.4.0)

**Goal:** Code organization and a rich standard library.

### 3.1 — Module System

- [ ] **`import` / `export`** — basic file-based modules
  ```
  # math_utils.zink
  fn square(x) { return x * x }
  export square

  # main.zink
  import { square } from "./math_utils.zink"
  say square(5)
  ```
- [ ] **Module resolution** — relative paths, standard library paths
- [ ] **Circular dependency detection** — error on circular imports
- [ ] **Module caching** — import once, reference many times
- [ ] **Wildcard import** — `import * from "./utils.zink"`

### 3.2 — Standard Library

Built-in modules that ship with the interpreter (`std:`):

| Module        | Functions / Purpose                                      |
|---------------|----------------------------------------------------------|
| `std:math`    | `pi`, `e`, `sin`, `cos`, `tan`, `log`, `log2`, `log10`  |
| `std:string`  | `starts_with`, `ends_with`, `replace`, `repeat`, `pad_left`, `pad_right`, `char_at`, `index_of` |
| `std:array`   | `flat`, `zip`, `unique`, `chunk`, `take`, `drop`, `find`, `find_index`, `every`, `some` |
| `std:map`     | `from_entries`, `merge`, `map_values`, `filter_entries`  |
| `std:io`      | `read_file`, `write_file`, `read_line`, `print` (no newline) |
| `std:fs`      | `exists`, `list_dir`, `is_file`, `is_dir`               |
| `std:json`    | `parse`, `stringify`                                     |
| `std:time`    | `now`, `sleep`, `elapsed`, `format_date`                 |
| `std:sys`     | `args`, `env`, `exit`, `platform`                        |

### 3.3 — Expanded Builtins

These become part of the core language (no import needed):

- [ ] `input(prompt)` — read user input from stdin
- [ ] `print(value)` — like `say` but without newline
- [ ] `format(template, ...args)` — explicit string formatting
- [ ] `to_json(value)` / `from_json(str)` — JSON serialization
- [ ] `typeof(value)` — alias for `type()` or replace it
- [ ] `is_nan(n)`, `is_finite(n)` — number checks
- [ ] `char_code(s)`, `from_char_code(n)` — character conversion

---

## Phase 4 — Performance & Bytecode (v0.5.0)

**Goal:** Replace tree-walk interpreter with a bytecode VM for 10–100x speed improvement.

### 4.1 — Bytecode Compiler

- [ ] **Define instruction set** — `LOAD_CONST`, `ADD`, `CALL`, `JUMP`, `RETURN`, etc.
- [ ] **Bytecode format** — compact binary representation of instructions
- [ ] **Compiler pass** — AST → bytecode (new `compiler.rs` module)
- [ ] **Constant pool** — store numbers, strings, function objects efficiently
- [ ] **Chunk / function objects** — each function compiled to its own bytecode chunk

### 4.2 — Virtual Machine

- [ ] **Stack-based VM** — push/pop operand stack, call frames
- [ ] **VM module** (`vm.rs`) — fetch-decode-execute loop
- [ ] **Garbage collector** — mark-and-sweep or reference counting for heap objects
- [ ] **Upvalue handling** — closures in bytecode world
- [ ] **Tail call optimization** — avoid stack overflow in recursive functions
- [ ] **Keep tree-walk as fallback** — `zink --tree-walk file.zink` for debugging

### 4.3 — Benchmarks

- [ ] **Benchmark suite** — fib(35), sort(10000), string concat, etc.
- [ ] **Compare tree-walk vs bytecode** — document speedup
- [ ] **Compare against Python, Lua, Ruby** — competitive analysis
- [ ] **CI benchmark tracking** — detect performance regressions

---

## Phase 5 — Type System & Safety (v0.6.0)

**Goal:** Optional type annotations for safety and documentation, without sacrificing simplicity.

### 5.1 — Optional Type Annotations

```
let x: number = 42
fn add(a: number, b: number) -> number {
  return a + b
}
let names: [string] = ["Alice", "Bob"]
```

- [ ] **Type syntax** — `: type` for variables and parameters, `-> type` for returns
- [ ] **Basic types** — `number`, `string`, `boolean`, `null`, `array`, `map`, `fn`
- [ ] **Array element types** — `[number]`, `[string]`
- [ ] **Map types** — `{string: number}`
- [ ] **Union types** — `number | null`, `string | number`
- [ ] **Type inference** — types optional; inferred when omitted
- [ ] **Type checker pass** — `lang/src/type_checker.rs` runs between parser and interpreter
- [ ] **Gradual typing** — typed and untyped code coexist freely

### 5.2 — Advanced Types (later)

- [ ] **Generics** — `fn map<T, U>(arr: [T], f: fn(T) -> U) -> [U]`
- [ ] **Type aliases** — `type Point = { x: number, y: number }`
- [ ] **Enum types** — `type Color = Red | Green | Blue`
- [ ] **Pattern matching** — `match value { 1 => "one", 2 => "two", _ => "other" }`

---

## Phase 6 — Tooling & Ecosystem (v0.7.0)

**Goal:** Professional developer tools that make Zink a joy to use.

### 6.1 — Language Server Protocol (LSP)

- [ ] **`zink-lsp` binary** — separate crate or integrated into main binary
- [ ] **Diagnostics** — real-time error highlighting in editor
- [ ] **Go to definition** — jump to function/variable declarations
- [ ] **Hover info** — show type and documentation on hover
- [ ] **Auto-completion** — builtins, variables, function names
- [ ] **Rename symbol** — rename across file
- [ ] **Document symbols** — outline view of functions and variables
- [ ] **VS Code extension integration** — connect extension to LSP

### 6.2 — Formatter

- [ ] **`zink fmt`** — auto-format Zink source code
- [ ] **Consistent style** — indentation, spacing, line length
- [ ] **Editor integration** — format on save in VS Code
- [ ] **CI check** — `zink fmt --check` in CI pipeline

### 6.3 — Linter

- [ ] **`zink lint`** — static analysis for common mistakes
- [ ] **Rules** — unused variables, unreachable code, shadowing warnings
- [ ] **Configurable** — enable/disable rules per project
- [ ] **Auto-fix** — `zink lint --fix` for auto-fixable issues

### 6.4 — Package Manager

- [ ] **`zink.toml`** — project manifest (name, version, dependencies)
- [ ] **`zink install`** — install packages from registry
- [ ] **`zink add <pkg>`** — add a dependency
- [ ] **Package registry** — central repository for Zink packages (later)
- [ ] **Local packages** — path-based dependencies for monorepos

### 6.5 — Debugger

- [ ] **`zink debug file.zink`** — step-through debugger in terminal
- [ ] **Breakpoints** — set line-based breakpoints
- [ ] **Step in/over/out** — navigate call stack
- [ ] **Variable inspection** — print variable values at breakpoints
- [ ] **DAP integration** — Debug Adapter Protocol for VS Code

---

## Phase 7 — Native Compilation (v0.8.0)

**Goal:** Compile Zink to native machine code for maximum performance.

### 7.1 — Compiler Backend

- [ ] **Target selection** — compile to native via LLVM, Cranelift, or custom backend
- [ ] **AOT compilation** — `zink build main.zink -o main` produces a native binary
- [ ] **IR (Intermediate Representation)** — lower AST to a typed IR before codegen
- [ ] **Optimization passes** — constant folding, dead code elimination, inlining
- [ ] **Cross-compilation** — build for different OS/arch from one machine

### 7.2 — FFI (Foreign Function Interface)

- [ ] **Call C functions** — `extern fn printf(fmt: string, ...)`
- [ ] **C library bindings** — link against system libraries
- [ ] **Struct layout** — compatible with C struct ABI
- [ ] **Callback support** — pass Zink functions to C code

### 7.3 — Execution Modes

After this phase, Zink supports three modes:

| Mode          | Command                  | Speed    | Use case           |
|---------------|--------------------------|----------|--------------------|
| Interpreted   | `zink run file.zink`     | 1x       | Development, REPL  |
| Bytecode VM   | `zink run file.zink`     | 10–100x  | Default runtime    |
| Native binary | `zink build file.zink`   | 500x+    | Production deploy  |

---

## Phase 8 — Self-Hosting (v0.9.0)

**Goal:** Write core Zink tooling in Zink itself — the ultimate language maturity milestone.

### 8.1 — Bootstrap Path

1. **Zink-in-Zink lexer** — rewrite `lexer.rs` logic in Zink
2. **Zink-in-Zink parser** — rewrite `parser.rs` in Zink
3. **Zink-in-Zink interpreter** — or bytecode compiler in Zink
4. **Self-hosting compiler** — Zink compiles itself
5. **Bootstrap test** — compiled-by-Zink compiler produces identical output to Rust compiler

### 8.2 — Prerequisites

The following language features are required before self-hosting is possible:

- File I/O (`read_file`, `write_file`)
- String manipulation (already mostly there)
- Maps / dictionaries (for symbol tables)
- Error handling (`try`/`catch`)
- Module system (to split the compiler across files)
- Large program support (remove step limit or make it configurable)
- Char-by-char string access (already exists via indexing)

### 8.3 — Milestones

- [ ] Zink lexer passes all tokenization tests when run in Zink
- [ ] Zink parser produces correct AST for all example programs
- [ ] Self-hosted interpreter runs `hello.zink` correctly
- [ ] Self-hosted compiler passes full test suite
- [ ] Bootstrap: self-hosted compiler compiles itself

---

## Phase 9 — Production Readiness (v1.0.0)

**Goal:** Stable, documented, tested, and ready for real-world use.

### 9.1 — Stability

- [ ] **Semantic versioning guarantee** — no breaking changes in 1.x
- [ ] **Language specification** — formal grammar, type rules, evaluation semantics
- [ ] **Backward compatibility tests** — ensure old programs still work
- [ ] **Deprecation policy** — warn for one minor version before removing features
- [ ] **Performance baselines** — documented performance characteristics

### 9.2 — Documentation

- [ ] **Complete language reference** — every feature, every edge case
- [ ] **Tutorial series** — beginner → intermediate → advanced
- [ ] **Cookbook** — common patterns and recipes
- [ ] **API docs for std library** — every module, every function
- [ ] **Playground examples** — 20+ covering all features
- [ ] **Video tutorials** (optional) — getting started, building projects

### 9.3 — Testing

- [ ] **90%+ code coverage** — lexer, parser, interpreter/VM, builtins
- [ ] **Fuzz testing** — random input → no crashes
- [ ] **Property-based testing** — invariants hold across random programs
- [ ] **Regression suite** — every fixed bug has a test
- [ ] **Cross-platform CI** — Linux, macOS, Windows on every PR
- [ ] **WASM tests** — verify interpreter works identically in WASM and native

### 9.4 — Ecosystem

- [ ] **Published on crates.io** — `cargo install zink`
- [ ] **Published on npm** — WASM package for Node.js
- [ ] **VS Code Marketplace** — extension published officially
- [ ] **Homebrew formula** — `brew install zink`
- [ ] **Winget package** — `winget install zink`
- [ ] **Docker image** — `docker run zink file.zink`
- [ ] **GitHub Action** — `uses: otabekoff/zink-action@v1`

### 9.5 — Community

- [ ] **GOVERNANCE.md** — project governance and decision process
- [ ] **RFC process** — for language-level changes
- [ ] **Discord / community chat** — real-time discussion
- [ ] **Blog** — release announcements, language design posts
- [ ] **Conference talks** (optional) — present Zink at meetups

---

## Version Summary

| Version   | Codename             | Theme                           | Key Deliverable                   |
|-----------|----------------------|---------------------------------|-----------------------------------|
| **v0.1.0** | *Spark*             | It works                        | ✅ Complete (current)             |
| **v0.2.0** | *Flow*              | Language completeness           | for-in, maps, break/continue     |
| **v0.3.0** | *Shield*            | Error handling & quality        | try/catch, assertions, tests     |
| **v0.4.0** | *Horizon*           | Modules & standard library      | import/export, std modules       |
| **v0.5.0** | *Volt*              | Bytecode VM                     | 10–100x performance              |
| **v0.6.0** | *Prism*             | Optional type system            | Gradual typing                   |
| **v0.7.0** | *Forge*             | Tooling & ecosystem             | LSP, formatter, package manager  |
| **v0.8.0** | *Thunder*           | Native compilation              | AOT compiler, FFI                |
| **v0.9.0** | *Mirror*            | Self-hosting                    | Zink compiles itself             |
| **v1.0.0** | *Zenith*            | Production stable               | Stable API, full ecosystem       |

---

## Design Principles for the Road

Every feature decision should be guided by the [Zen of Zink](README.md#the-zen-of-zink):

1. **Readable first** — if it's hard to read, don't add it
2. **One obvious way** — don't give three syntaxes for the same thing
3. **Batteries included, not batteries required** — useful out of the box, not bloated
4. **Errors are not exceptional** — make error handling simple and natural
5. **Gradual complexity** — simple things should be simple, complex things should be possible

When in doubt: would a 14-year-old writing their first program understand this? If not, simplify.

---

## Contributing to the Roadmap

This roadmap is aspirational and flexible. Priorities may shift based on:

- Community feedback and feature requests
- Technical discoveries during implementation
- Ecosystem demands (what users actually build)

To propose changes to the roadmap, open an issue with the `roadmap` label or submit an RFC.
