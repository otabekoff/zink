# Contributing to Zink

Thank you for your interest in contributing to Zink! This guide will help you get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Commit Convention](#commit-convention)
- [Pull Request Process](#pull-request-process)
- [Local Build Commands](#local-build-commands)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/zink.git`
3. Create a branch: `git checkout -b feat/my-feature`
4. Make your changes
5. Push and open a pull request

## Development Setup

### Prerequisites

- **Rust** (stable, 1.82+): [rustup.rs](https://rustup.rs/)
- **Node.js** (20+) or **Bun** (1.0+): for the IDE and docs
- **Git**

### Install & Build

```bash
# Clone
git clone https://github.com/otabekoff/zink.git
cd zink

# ─── Interpreter (Rust) ───
cd lang
cargo build
cargo test
cargo run -- examples/hello.zink

# ─── Playground (React + Vite) ───
cd ../playground
npm install
npm run dev          # dev server at localhost:5173
npm run build        # production build
npm run lint         # ESLint

# ─── Documentation (VitePress) ───
cd ../docs
bun install          # or: npm install
bun run dev          # dev server at localhost:5173
bun run build        # production build
```

## Project Structure

```
zink/
├── lang/                   # Rust interpreter
│   ├── src/
│   │   ├── main.rs         # CLI entry point + REPL
│   │   ├── lib.rs          # Library + WASM bindings
│   │   ├── lexer.rs        # Tokenizer
│   │   ├── parser.rs       # Recursive-descent parser → AST
│   │   └── interpreter.rs  # Tree-walk interpreter + builtins
│   ├── editors/            # TextMate grammar
│   └── examples/           # Example .zink programs
├── playground/             # Browser playground
│   ├── src/
│   │   ├── ZinkIDE.tsx     # Main React component
│   │   ├── zink-interpreter.ts   # WASM wrapper
│   │   └── main.tsx        # Entry point
│   └── public/
├── docs/                   # VitePress documentation
│   ├── guide/              # Tutorial pages
│   ├── reference/          # API reference pages
│   └── .vitepress/         # VitePress config
├── extension/              # VS Code extension
├── .github/                # CI/CD workflows & templates
├── LICENSE                 # MIT License
├── CONTRIBUTING.md         # This file
├── CODE_OF_CONDUCT.md      # Contributor Covenant
├── SECURITY.md             # Security policy
└── CHANGELOG.md            # Release history
```

## Making Changes

### For the interpreter (`lang/`)

- Follow Rust conventions (`rustfmt`, `clippy`)
- Add test cases for new features
- Update examples if adding new syntax
- Run `cargo test` before submitting

### For the playground (`playground/`)

- WASM interpreter is built from `lang/` via `npm run build:wasm`
- Run `npm run lint` and `npm run build` before submitting
- Test in multiple browsers

### For the docs (`docs/`)

- Follow existing page structure
- Include code examples for every feature
- Run `bun run build` to check for broken links

## Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add while loop support
fix: correct string interpolation edge case
docs: update built-in functions reference
chore: update CI workflow
refactor: simplify parser precedence climbing
test: add lexer edge case tests
```

**Types**: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `style`, `perf`, `ci`

**Scope** (optional): `lang`, `ide`, `docs`

```
feat(lang): add pattern matching
fix(ide): fix syntax highlighting for numbers
docs: add arrays tutorial
```

## Pull Request Process

1. **Branch** from `main` — use `feat/`, `fix/`, or `docs/` prefix
2. **Keep PRs focused** — one feature or fix per PR
3. **Write a clear description** — what changed and why
4. **Pass CI** — all checks must be green
5. **Request review** — tag a maintainer
6. **Squash merge** — we squash into a single commit on merge

## Local Build Commands

| Project   | Command              | Description                    |
|-----------|----------------------|--------------------------------|
| `lang`       | `cargo build`      | Build debug binary             |
| `lang`       | `cargo build --release` | Build optimized binary    |
| `lang`       | `cargo test`       | Run test suite                 |
| `lang`       | `cargo run -- FILE` | Run a .zink file              |
| `lang`       | `cargo run`        | Start REPL                     |
| `lang`       | `cargo clippy`     | Lint Rust code                 |
| `playground` | `npm run dev`      | Dev server (hot reload)        |
| `playground` | `npm run build`    | Production build               |
| `playground` | `npm run preview`  | Preview production build       |
| `playground` | `npm run lint`     | ESLint                         |
| `docs`       | `bun run dev`      | Docs dev server                |
| `docs`       | `bun run build`    | Docs production build          |
| `docs`       | `bun run preview`  | Preview docs build             |

## Questions?

Open an issue or start a discussion on GitHub. We're happy to help!
