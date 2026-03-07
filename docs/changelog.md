# Changelog

All notable changes to Zink are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-03-07

### Added

- **Language core**: variables (`let`), functions (`fn`), `return`, `if`/`else`, `while`, `loop N times`, `say`
- **Types**: numbers (f64), strings with `{expr}` interpolation, booleans, null, arrays, first-class functions
- **Operators**: arithmetic (`+`, `-`, `*`, `/`, `%`), comparison (`==`, `!=`, `<`, `>`, `<=`, `>=`), logical (`and`, `or`, `not`)
- **30+ built-in functions**: math, string manipulation, array operations, higher-order functions
- **CLI**: file execution, interactive REPL with multi-line support, stdin pipe mode
- **Browser playground** (`playground/`): React + Vite app with syntax highlighting, 8 examples, docs sidebar
- **Documentation** (`docs/`): VitePress site with guide, reference, and API docs
- **9 example programs**: hello world, variables, functions, FizzBuzz, Fibonacci, arrays, higher-order functions, prime numbers
- **CI/CD**: GitHub Actions for Rust build/test, IDE deployment to GitHub Pages, docs deployment
