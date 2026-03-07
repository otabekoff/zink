# Getting Started

## Installation

### From source (Rust)

```bash
# Clone the repository
git clone https://github.com/otabekoff/zink.git
cd zink/lang

# Build the release binary
cargo build --release

# The binary is at target/release/zink (or zink.exe on Windows)
```

### Run a file

```bash
./target/release/zink examples/hello.zink
```

### Start the REPL

```bash
./target/release/zink
# ⚡ Zink v0.1.0 — REPL
# >>> say "Hello!"
# Hello!
```

### Pipe from stdin

```bash
echo 'say "Hello from pipe!"' | ./target/release/zink -
```

## Try Online

No installation needed — use the [Zink Playground](https://zink-lang.github.io/playground/) to write and run Zink code directly in your browser.

## Your First Program

Create a file called `hello.zink`:

```zink
# hello.zink
let name = "World"
say "Hello, {name}!"

fn add(a, b) {
  return a + b
}

say "2 + 3 = {add(2, 3)}"
```

Run it:

```bash
./target/release/zink hello.zink
# Hello, World!
# 2 + 3 = 5
```

## Project Structure

```
zink/
├── lang/               # Rust interpreter
│   ├── src/
│   │   ├── main.rs     # CLI + REPL
│   │   ├── lib.rs      # Library + WASM bindings
│   │   ├── lexer.rs    # Tokenizer
│   │   ├── parser.rs   # AST builder
│   │   └── interpreter.rs  # Evaluator
│   └── examples/       # Example .zink programs
├── playground/         # Browser playground (React + Vite)
├── docs/               # This documentation (VitePress)
└── extension/          # VS Code extension
```

## Next Steps

- [Variables](/guide/variables) — Learn about `let` bindings
- [Functions](/guide/functions) — Define and call functions
- [Control Flow](/guide/control-flow) — `if`, `while`, and `loop`
- [Built-in Functions](/reference/builtins) — 30+ functions included
