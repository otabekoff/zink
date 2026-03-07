# Zink Language

The Zink interpreter — a tree-walk interpreter for the Zink scripting language, written in Rust.

## Features

- Lexer → Parser → Interpreter pipeline
- Dynamic typing: numbers, strings, booleans, null, arrays, functions
- String interpolation with `{expr}` syntax
- 30+ built-in functions (math, strings, arrays, higher-order)
- Interactive REPL with multi-line support
- Clear, line-numbered error messages

## Build

```bash
cargo build --release
```

## Usage

```bash
# Run a file
./target/release/zink examples/hello.zink

# Start the REPL
./target/release/zink

# Pipe from stdin
echo 'say "Hello!"' | ./target/release/zink -
```

## Examples

The `examples/` directory contains sample programs:

| File                          | Description                    |
|-------------------------------|--------------------------------|
| `hello.zink`                  | Hello world + closures         |
| `hello_world.zink`            | Basic output and variables     |
| `variables_and_math.zink`     | Arithmetic operations          |
| `functions.zink`              | Function definitions, recursion|
| `fizzbuzz.zink`               | Classic FizzBuzz               |
| `fibonacci.zink`              | Recursive Fibonacci            |
| `arrays_and_loops.zink`       | Array ops + bubble sort        |
| `higher_order_functions.zink` | map, filter, reduce            |
| `prime_numbers.zink`          | Prime sieve                    |

## Architecture

```
src/
├── main.rs         # CLI entry point + REPL
├── lexer.rs        # Tokenizer (source → tokens)
├── parser.rs       # Recursive-descent parser (tokens → AST)
└── interpreter.rs  # Tree-walk interpreter (AST → output)
```

## Testing

```bash
cargo test
cargo clippy
cargo fmt --check
```

## License

MIT — see [LICENSE](../LICENSE)
