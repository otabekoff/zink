# ⚡ Zink

**Say what you mean. Mean what you say.**

A fast, readable scripting language built in Rust.

[![CI — Lang](https://img.shields.io/github/actions/workflow/status/otabekoff/zink/ci-lang.yml?branch=main&label=Lang&logo=rust&logoColor=white)](https://github.com/otabekoff/zink/actions/workflows/ci-lang.yml)
[![CI — Playground](https://img.shields.io/github/actions/workflow/status/otabekoff/zink/ci-ide.yml?branch=main&label=Playground&logo=react)](https://github.com/otabekoff/zink/actions/workflows/ci-ide.yml)
[![CI — Docs](https://img.shields.io/github/actions/workflow/status/otabekoff/zink/ci-docs.yml?branch=main&label=Docs&logo=vitepress)](https://github.com/otabekoff/zink/actions/workflows/ci-docs.yml)
[![Deploy](https://img.shields.io/github/actions/workflow/status/otabekoff/zink/deploy.yml?branch=main&label=Deploy&logo=github)](https://github.com/otabekoff/zink/actions/workflows/deploy.yml)
[![Extension](https://img.shields.io/badge/Extension-v0.1.0-blue?logo=visualstudiocode&logoColor=white)](https://github.com/otabekoff/zink/releases/latest)
[![Release](https://img.shields.io/github/v/release/otabekoff/zink?label=Release&logo=github&color=blue)](https://github.com/otabekoff/zink/releases/latest)
[![License](https://img.shields.io/github/license/otabekoff/zink?label=License&color=green)](LICENSE)

```ruby
# Hello, Zink!
fn greet(name) {
  say "Hello, {name}! Welcome to Zink."
}

greet("World")

let nums = [1, 2, 3, 4, 5]
let doubled = map(nums, fn(x) { return x * 2 })
say "Doubled: {doubled}"
```

## Philosophy

Zink is a language for people who value clarity over cleverness.

It was built with one question: **what if a programming language was designed to be read?** Not by compilers, not by experts — by humans. By beginners writing their first program and by seasoned developers reading someone else's code at 2 AM.

Zink doesn't try to be everything. It tries to be obvious.

### The Zen of Zink

```
1.  Readable is better than clever.
2.  Obvious is better than implicit.
3.  One way is better than many ways.
4.  Simple things should be simple.
5.  Complex things should be possible.
6.  Errors should explain, not blame.
7.  Batteries included, not batteries required.
8.  A program is a story — it should read like one.
9.  If you need a comment to explain the syntax, the syntax is wrong.
10. Say what you mean. Mean what you say.
```

### Design Principles

- **No semicolons** — line breaks are enough.
- **No type annotations** (for now) — types are for the computer, not the writer.
- **String interpolation everywhere** — `"Hello, {name}!"` just works.
- **Functions are values** — pass them, return them, store them.
- **Braces mean blocks** — no ambiguity about scope.
- **`say` instead of `print`** — because programs should speak, not print.
- **`loop 5 times`** — reads like English, runs like code.

## Why Zink?

- **Readable** — Clean syntax, no semicolons, no type annotations
- **Fast** — Rust-powered interpreter, instant startup
- **Beginner-friendly** — `say "Hello!"` is a valid program
- **Batteries included** — 28 built-in functions out of the box
- **Try online** — Full browser playground at [otabekoff.github.io/zink/playground](https://otabekoff.github.io/zink/playground/)

## Quick Start

```bash
# Build from source
git clone https://github.com/otabekoff/zink.git
cd zink/lang
cargo build --release

# Run a file
./target/release/zink examples/hello.zink

# Start the REPL
./target/release/zink
```

Or use the install script:

```bash
# macOS / Linux
./setup.sh

# Windows (PowerShell)
.\setup.ps1
```

## Language Features

| Feature            | Syntax                          |
|--------------------|----------------------------------|
| Variables          | `let x = 42`                    |
| Print              | `say "Hello, {name}!"`          |
| If / Else          | `if x > 0 { ... } else { ... }` |
| While loop         | `while x < 10 { ... }`         |
| Counted loop       | `loop 5 times { ... }`          |
| Functions          | `fn add(a, b) { return a + b }` |
| Arrays             | `let arr = [1, 2, 3]`           |
| String interp.     | `"Value is {expr}"`             |
| Higher-order fns   | `map(arr, fn(x) { return x * 2 })` |
| Comments           | `# this is a comment`           |

## Built-in Functions

```
len(x)      push(arr, v)   pop(arr)     str(v)       num(s)
type(v)     floor(n)       ceil(n)      round(n)     abs(n)
sqrt(n)     pow(b, e)      max(a, b)    min(a, b)    random()
range(s,e)  contains(c,x)  join(arr,s)  split(s,sep) upper(s)
lower(s)    trim(s)        slice(a,s,e) reverse(arr) sort(arr)
map(arr,fn) filter(arr,fn) reduce(arr,fn,init)
```

## Project Structure

```
zink/
├── lang/          Rust interpreter (lexer → parser → tree-walk interpreter)
├── playground/    Browser playground (React + Vite)
├── docs/          Documentation site (VitePress)
├── extension/     VS Code extension (syntax highlighting)
└── .github/       CI/CD workflows & templates
```

## Local Development

| Project     | Command                    | Description              |
|-------------|----------------------------|--------------------------|
| **Lang**    | `cargo build --release`    | Build optimized binary   |
| **Lang**    | `cargo test`               | Run tests                |
| **Lang**    | `cargo run -- FILE`        | Run a .zink file         |
| **Playground** | `bun run dev`           | Dev server (hot reload)  |
| **Playground** | `bun run build`         | Production build         |
| **Playground** | `bun run lint`          | ESLint                   |
| **Extension**  | `npm run compile`       | Compile extension        |
| **Extension**  | `npm run package`       | Package VSIX             |
| **Docs**    | `bun run dev`              | Docs dev server          |
| **Docs**    | `bun run build`            | Production build         |

See [DEVELOPMENT.md](DEVELOPMENT.md) for the complete developer guide, workflow, and release process.

## Roadmap

Zink v0.1.0 is the first public release. The path to v1.0 includes:

- **v0.2** — `for..in` loops, maps/dictionaries, `break`/`continue`, compound assignment
- **v0.3** — `try`/`catch` error handling, assertions, test framework
- **v0.4** — Module system (`import`/`export`), standard library
- **v0.5** — Bytecode VM (10–100x performance)
- **v0.6** — Optional type annotations (gradual typing)
- **v0.7** — LSP, formatter, linter, package manager
- **v0.8** — Native compilation (AOT), FFI
- **v0.9** — Self-hosting compiler (Zink compiles itself)
- **v1.0** — Stable, production-ready, full ecosystem

See [ROADMAP.md](ROADMAP.md) for the detailed plan with every task.

## Documentation

Read the full docs at [otabekoff.github.io/zink](https://otabekoff.github.io/zink/) or browse [docs/](docs/) locally.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, commit conventions, and PR guidelines.

## License

MIT © 2026-present Otabek Sadiridinov — see [LICENSE](LICENSE)
