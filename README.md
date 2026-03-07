# ⚡ Zink

A fast, readable scripting language built in Rust.

[![CI — Lang](https://github.com/otabekoff/zink/actions/workflows/ci-lang.yml/badge.svg)](https://github.com/otabekoff/zink/actions/workflows/ci-lang.yml)
[![CI — IDE](https://github.com/otabekoff/zink/actions/workflows/ci-ide.yml/badge.svg)](https://github.com/otabekoff/zink/actions/workflows/ci-ide.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

```zink
# Hello, Zink!
fn greet(name) {
  say "Hello, {name}! Welcome to Zink."
}

greet("World")

let nums = [1, 2, 3, 4, 5]
let doubled = map(nums, fn(x) { return x * 2 })
say "Doubled: {doubled}"
```

## Why Zink?

- **Readable** — Clean syntax, no semicolons, no type annotations
- **Fast** — Rust-powered interpreter, instant startup
- **Beginner-friendly** — `say "Hello!"` is a valid program
- **Batteries included** — 30+ built-in functions out of the box
- **Try online** — Full browser playground at [zink-lang.github.io/playground](https://zink-lang.github.io/playground/)

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
| **IDE**     | `npm run dev`              | Dev server (hot reload)  |
| **IDE**     | `npm run build`            | Production build         |
| **IDE**     | `npm run lint`             | ESLint                   |
| **Docs**    | `bun run dev`              | Docs dev server          |
| **Docs**    | `bun run build`            | Production build         |

## Documentation

Read the full docs at [zink-lang.github.io/docs](https://zink-lang.github.io/docs/) or browse [docs/](docs/) locally.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, commit conventions, and PR guidelines.

## License

MIT © 2026-present Otabek Sadiridinov — see [LICENSE](LICENSE)
