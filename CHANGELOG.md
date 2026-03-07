# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-03-07

### Added

#### Language (`lang`)
- Core language: variables (`let`), functions (`fn`), `return`, `if`/`else`, `while`, `loop N times`, `say`
- Data types: numbers (f64), strings with `{expr}` interpolation, booleans, null, arrays, first-class functions
- Operators: arithmetic (`+`, `-`, `*`, `/`, `%`), comparison (`==`, `!=`, `<`, `>`, `<=`, `>=`), logical (`and`, `or`, `not`)
- 30+ built-in functions: math, string manipulation, array operations, higher-order functions (`map`, `filter`, `reduce`)
- CLI with three modes: file execution, interactive REPL (multi-line support), stdin pipe
- 9 example programs demonstrating all language features

#### Playground (`playground`)
- Browser-based IDE with syntax highlighting and line numbers
- 8 runnable examples with descriptions
- Integrated documentation sidebar
- Keyboard shortcuts (Ctrl+Enter to run, Tab for indentation)
- Execution timer and error display

#### Documentation (`docs`)
- VitePress documentation site
- Getting started guide
- Language tutorials: variables, functions, control flow, arrays, strings, higher-order functions
- Reference pages: syntax, built-in functions, operators, types
- Local search support

#### Infrastructure
- GitHub Actions CI/CD: Rust build/test, IDE deployment, docs deployment
- Dependabot configuration for automated dependency updates
- Issue and PR templates
- MIT license, Code of Conduct, Contributing guide, Security policy
