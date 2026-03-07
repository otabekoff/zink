# Development Guide

Everything you need to work with the Zink monorepo — build commands, workflows, release process, and conventions.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Repository Structure](#repository-structure)
- [Quick Setup](#quick-setup)
- [Working with Each Project](#working-with-each-project)
  - [Lang (Rust Interpreter)](#lang-rust-interpreter)
  - [Playground (React + Vite)](#playground-react--vite)
  - [Docs (VitePress)](#docs-vitepress)
  - [Extension (VS Code)](#extension-vs-code)
- [WASM Build Pipeline](#wasm-build-pipeline)
- [Git Workflow](#git-workflow)
- [CI/CD Pipelines](#cicd-pipelines)
- [Release Process](#release-process)
- [Dependency Management](#dependency-management)
- [Order of Operations](#order-of-operations)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

| Tool            | Version   | Purpose                          | Install                                  |
|-----------------|-----------|----------------------------------|------------------------------------------|
| **Rust**        | stable 1.82+ | Lang interpreter & WASM build | [rustup.rs](https://rustup.rs/)          |
| **wasm-pack**   | 0.12+     | Compile Rust to WASM             | `cargo install wasm-pack`                |
| **Bun**         | 1.0+      | Docs & playground package manager | [bun.sh](https://bun.sh/)              |
| **Node.js**     | 20+       | VS Code extension (npm)          | [nodejs.org](https://nodejs.org/)        |
| **Git**         | 2.40+     | Version control                  | System package manager                   |
| **VS Code**     | 1.109+    | Extension development & testing  | [code.visualstudio.com](https://code.visualstudio.com/) |

**Optional:**

| Tool            | Purpose                        | Install                            |
|-----------------|--------------------------------|------------------------------------|
| **cargo-clippy** | Rust linting (included with rustup) | `rustup component add clippy` |
| **cargo-fmt**   | Rust formatting                | `rustup component add rustfmt`     |
| **gh**          | GitHub CLI (for releases)      | [cli.github.com](https://cli.github.com/) |

---

## Repository Structure

```
zink/
├── lang/                   # Rust interpreter (core project)
│   ├── src/
│   │   ├── main.rs         # CLI — file runner, REPL, stdin pipe
│   │   ├── lib.rs          # Library crate + WASM bindings
│   │   ├── lexer.rs        # Tokenizer (source → tokens)
│   │   ├── parser.rs       # Parser (tokens → AST)
│   │   └── interpreter.rs  # Tree-walk interpreter + builtins
│   ├── examples/           # 9 example .zink programs
│   └── Cargo.toml
│
├── playground/             # Browser IDE (React 19 + Vite 8)
│   ├── src/
│   │   ├── ZinkIDE.tsx     # Main IDE component (examples, docs, editor)
│   │   ├── zink-interpreter.ts  # WASM loader & bridge
│   │   └── main.tsx        # React entry point
│   ├── public/             # Static assets (WASM files go here)
│   └── package.json
│
├── docs/                   # Documentation (VitePress 2)
│   ├── guide/              # Tutorial pages (9 pages)
│   ├── reference/          # API reference (4 pages)
│   ├── .vitepress/
│   │   ├── config.ts       # Site config, nav, sidebar
│   │   └── theme/          # Custom theme + Zink grammar
│   └── package.json
│
├── extension/              # VS Code extension
│   ├── src/extension.ts    # Extension entry — commands, REPL
│   ├── syntaxes/           # TextMate grammar
│   ├── snippets/           # Code snippets
│   └── package.json
│
├── .github/
│   ├── workflows/          # CI/CD (5 workflows)
│   ├── ISSUE_TEMPLATE/     # Bug report & feature request
│   ├── PULL_REQUEST_TEMPLATE.md
│   ├── dependabot.yml
│   └── FUNDING.yml
│
├── setup.ps1               # Windows install script
├── setup.sh                # macOS/Linux install script
├── DEVELOPMENT.md          # This file
├── ROADMAP.md              # v0.1.0 → v1.0 plan
├── CONTRIBUTING.md         # Contributor guide
├── CHANGELOG.md            # Release notes
├── CODE_OF_CONDUCT.md      # Contributor Covenant
├── SECURITY.md             # Security policy
├── LICENSE                 # MIT
└── README.md               # Project overview
```

---

## Quick Setup

### One-line install (interpreter + extension)

```bash
# macOS / Linux
./setup.sh

# Windows (PowerShell)
.\setup.ps1
```

### Manual setup

```bash
git clone https://github.com/otabekoff/zink.git
cd zink

# Build the interpreter
cd lang && cargo build --release && cd ..

# Install globally (adds `zink` to PATH)
cd lang && cargo install --path . --force && cd ..

# Start the playground
cd playground && bun install && bun run dev && cd ..

# Start the docs
cd docs && bun install && bun run dev && cd ..
```

---

## Working with Each Project

### Lang (Rust Interpreter)

The core of Zink. Everything else depends on this.

```bash
cd lang
```

| Command                        | Description                                |
|--------------------------------|--------------------------------------------|
| `cargo build`                  | Build debug binary                         |
| `cargo build --release`        | Build optimized binary                     |
| `cargo run -- examples/hello.zink` | Run a .zink file                      |
| `cargo run`                    | Start the REPL                             |
| `cargo test`                   | Run test suite                             |
| `cargo clippy`                 | Lint (treat warnings as errors in CI)      |
| `cargo fmt --check`            | Check formatting                           |
| `cargo fmt`                    | Auto-format code                           |
| `cargo install --path . --force` | Install `zink` binary to `~/.cargo/bin/` |

**Architecture:** `source text → Lexer → tokens → Parser → AST → Interpreter → output`

**Must-do before committing:**

1. `cargo fmt` — format code
2. `cargo clippy` — no warnings
3. `cargo test` — all tests pass
4. Run all examples: `for f in examples/*.zink; do cargo run -- "$f"; done`

### Playground (React + Vite)

Browser-based Zink IDE that runs via WASM.

```bash
cd playground
```

| Command            | Description                                     |
|--------------------|-------------------------------------------------|
| `bun install`      | Install dependencies                            |
| `bun run dev`      | Dev server at `localhost:5173` (hot reload)      |
| `bun run build`    | Production build to `dist/`                      |
| `bun run preview`  | Preview production build locally                 |
| `bun run lint`     | ESLint check                                     |
| `bun run build:wasm` | Build WASM from lang + copy to `public/`       |

**Key files:**

- `src/ZinkIDE.tsx` — The entire IDE (editor, examples, docs sidebar, output panel)
- `src/zink-interpreter.ts` — Loads WASM and exposes `initZink()` / `runZink()`
- `public/zink_lang_bg.wasm` — Pre-built WASM binary (committed to repo)
- `public/zink_lang.js` — WASM JS glue (committed to repo)

**Must-do before committing:**

1. `bun run lint` — no lint errors
2. `bun run build` — production build succeeds

### Docs (VitePress)

Documentation site with guide, reference, and tutorials.

```bash
cd docs
```

| Command            | Description                          |
|--------------------|--------------------------------------|
| `bun install`      | Install dependencies                 |
| `bun run dev`      | Dev server at `localhost:5173/zink/`  |
| `bun run build`    | Production build to `.vitepress/dist/` |
| `bun run preview`  | Preview production build             |

**Key files:**

- `.vitepress/config.ts` — Site config, nav, sidebar, base URL (`/zink/`)
- `.vitepress/theme/` — Custom theme, styles, Zink TextMate grammar for Shiki
- `guide/` — Tutorial pages (variables, functions, control-flow, etc.)
- `reference/` — API reference (syntax, types, operators, builtins)
- `index.md` — Landing page (hero, features)

**Must-do before committing:**

1. `bun run build` — no broken links or build errors
2. Code examples must be valid Zink that actually runs

### Extension (VS Code)

Syntax highlighting, snippets, and REPL integration.

```bash
cd extension
```

| Command               | Description                                   |
|-----------------------|-----------------------------------------------|
| `npm install`         | Install dependencies                          |
| `npm run compile`     | Compile TypeScript to `out/`                  |
| `npm run watch`       | Watch mode (recompile on save)                |
| `npm run lint`        | ESLint check                                  |
| `npm run package`     | Package as `.vsix`                            |
| `npm run install-ext` | Package + install in your VS Code             |
| `npm run test`        | Run extension tests                           |

**Key files:**

- `src/extension.ts` — Entry point (commands: run file, run selection, open REPL)
- `syntaxes/zink.tmLanguage.json` — TextMate grammar (shared with docs Shiki)
- `snippets/zink.json` — 11 code snippets
- `language-configuration.json` — Bracket pairs, comments, auto-close
- `images/` — Icons (icon.png, icon-dark.svg, icon-light.svg)

**Must-do before committing:**

1. `npm run compile` — no TypeScript errors
2. `npm run lint` — no lint errors
3. Test in VS Code: `F5` to launch Extension Development Host

---

## WASM Build Pipeline

The playground runs the Zink interpreter as WebAssembly. Here's how to rebuild it:

```bash
# 1. Install wasm-pack (one time)
cargo install wasm-pack

# 2. Build WASM from the lang crate
cd lang
wasm-pack build --target web --out-dir pkg

# 3. Copy artifacts to playground
cp pkg/zink_lang_bg.wasm ../playground/public/
cp pkg/zink_lang.js ../playground/src/
```

Or use the convenience script:

```bash
cd playground
bun run build:wasm
```

**When to rebuild WASM:**

- After any change to `lang/src/*.rs`
- After adding/removing builtins
- After changing the `run_zink()` WASM API in `lib.rs`

The WASM files in `playground/public/` are committed to the repo so the playground CI doesn't need Rust.

---

## Git Workflow

### Branch naming

```
feat/description       # New features
fix/description        # Bug fixes
docs/description       # Documentation only
chore/description      # Maintenance, deps, CI
refactor/description   # Code restructuring
```

### Commit convention

[Conventional Commits](https://www.conventionalcommits.org/):

```
feat(lang): add for-each loop
fix(lang): correct string slice bounds
docs: update builtins reference
chore: bump dependencies
ci: add WASM build step
refactor(lang): simplify parser precedence
```

**Types:** `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `style`, `perf`, `ci`
**Scopes:** `lang`, `playground`, `docs`, `extension` (optional)

### Branch protection (`main`)

- No force pushes
- No direct deletions
- PRs should pass CI before merge
- Squash merge for clean history

### Day-to-day workflow

```bash
# 1. Create a feature branch
git checkout -b feat/for-loop

# 2. Make changes, commit often
git add -A && git commit -m "feat(lang): add for-each loop"

# 3. Push and create PR
git push -u origin feat/for-loop
# Open PR on GitHub

# 4. After review, squash merge via GitHub UI

# 5. Pull latest main
git checkout main && git pull
```

---

## CI/CD Pipelines

Five workflows in `.github/workflows/`:

| Workflow        | File              | Trigger                          | What it does                           |
|-----------------|-------------------|----------------------------------|----------------------------------------|
| **CI Lang**     | `ci-lang.yml`     | Push/PR to `lang/**`             | fmt, clippy, build, test, run examples (3 OS) |
| **CI Playground** | `ci-ide.yml`    | Push/PR to `playground/**`       | lint, type check, build                |
| **CI Docs**     | `ci-docs.yml`     | Push/PR to `docs/**`             | build docs                             |
| **Deploy**      | `deploy.yml`      | Push to `main` (docs/playground) | Build docs + playground → GitHub Pages |
| **Release**     | `release.yml`     | Push `v*` tag                    | Create release + build binaries (3 OS) + VSIX |

All workflows also support `workflow_dispatch` (manual trigger from GitHub Actions UI).

---

## Release Process

### Step-by-step

```bash
# 1. Make sure main is clean and CI is green
git checkout main
git pull

# 2. Update version numbers (all must match)
#    - lang/Cargo.toml          → version = "X.Y.Z"
#    - playground/package.json   → "version": "X.Y.Z"
#    - docs/package.json         → (no version field, skip)
#    - extension/package.json    → "version": "X.Y.Z"
#    - CHANGELOG.md              → Add new section

# 3. Commit version bump
git add -A
git commit -m "chore: bump version to vX.Y.Z"
git push

# 4. Tag the release
git tag vX.Y.Z
git push origin vX.Y.Z

# 5. The release.yml workflow will automatically:
#    - Create a GitHub Release
#    - Build binaries for Linux x64, macOS ARM64, Windows x64
#    - Build and upload the VS Code extension .vsix
```

### Version locations (must stay in sync)

| File                      | Field                | Example       |
|---------------------------|----------------------|---------------|
| `lang/Cargo.toml`        | `version`            | `"0.1.0"`     |
| `playground/package.json` | `"version"`          | `"0.1.0"`     |
| `extension/package.json`  | `"version"`          | `"0.1.0"`     |
| `lang/src/main.rs`       | REPL banner string   | `"v0.1.0"`    |
| `playground/src/ZinkIDE.tsx` | Footer text       | `"v0.1.0"`    |
| `CHANGELOG.md`           | Latest section header | `[0.1.0]`    |
| `README.md`              | Extension badge      | `v0.1.0`      |

### Pre-release tags

Use semver pre-release suffixes — the release workflow auto-detects them:

```
v0.2.0-alpha.1    # Alpha (early preview)
v0.2.0-beta.1     # Beta (feature-complete, testing)
v0.2.0-rc.1       # Release candidate
v0.2.0            # Stable release
```

---

## Dependency Management

### Automated (Dependabot)

Dependabot is configured in `.github/dependabot.yml` to check:

- `lang/` — Cargo crates (weekly)
- `playground/` — npm packages (weekly)
- `docs/` — npm packages (weekly)
- `extension/` — npm packages (weekly)
- `.github/workflows/` — GitHub Actions (weekly)

### Manual updates

```bash
# Rust
cd lang && cargo update

# Playground
cd playground && bunx npm-check-updates -u && bun install

# Docs
cd docs && bunx npm-check-updates -u && bun install

# Extension
cd extension && npx npm-check-updates -u && npm install
```

---

## Order of Operations

### Adding a new language feature

1. **Lexer** (`lexer.rs`) — Add new `TokenKind` variant, update `tokenize()` or `read_ident()`
2. **Parser** (`parser.rs`) — Add new `Stmt`/`Expr` variant, add parsing rule
3. **Interpreter** (`interpreter.rs`) — Add execution logic in `exec_stmt()` or `eval_expr()`
4. **Tests** — Add unit tests, update example files
5. **Examples** — Add or update `.zink` files demonstrating the feature
6. **WASM** — Rebuild: `cd playground && bun run build:wasm`
7. **Playground** — Update examples in `ZinkIDE.tsx` if relevant
8. **Docs** — Add/update guide page, update reference pages
9. **Extension** — Update TextMate grammar, snippets if new syntax
10. **Commit** — Conventional commit: `feat(lang): add for-each loop`

### Adding a new builtin function

1. **Interpreter** — Register in `register_builtins()`, implement in `call_builtin()`
2. **Tests** — Test the new builtin
3. **WASM** — Rebuild
4. **Playground** — Update builtins list in docs sidebar, update syntax highlighter regex
5. **Docs** — Update `reference/builtins.md`, relevant guide pages
6. **Extension** — Update TextMate grammar to highlight the new builtin name

### Fixing a bug

1. Fix in the appropriate `lang/src/*.rs` file
2. Add a test that reproduces and verifies the fix
3. Rebuild WASM if it affects runtime behavior
4. Update docs if the fix changes documented behavior
5. Commit: `fix(lang): description of the fix`

---

## Troubleshooting

### `cargo build` fails with edition errors

Make sure you have Rust 1.82+ (edition 2024):

```bash
rustup update stable
```

### WASM build fails

```bash
# Install wasm-pack
cargo install wasm-pack

# Clean and rebuild
cd lang
cargo clean
wasm-pack build --target web --out-dir pkg
```

### Playground shows "Loading WASM..." forever

The WASM files might be missing from `public/`. Rebuild:

```bash
cd playground
bun run build:wasm
```

### Docs dev server 404s

Make sure you're accessing the right base URL: `http://localhost:5173/zink/` (not just `/`).

### Extension not activating

1. Make sure a `.zink` file is open
2. Check Output panel → "Zink" channel for errors
3. Try `Developer: Reload Window` from command palette

### CI failing on format check

```bash
cd lang
cargo fmt
```

Then commit the formatted changes.
