# Zink — Copilot Instructions

## What This Is

Zink is a scripting language built in Rust. Monorepo with five projects:

| Project       | Path          | Stack                        |
|---------------|---------------|------------------------------|
| Language      | `lang/`       | Rust 2024, tree-walk interp  |
| Playground    | `playground/` | React 19 + TypeScript + Vite |
| Docs          | `docs/`       | VitePress 2 + Bun            |
| Extension     | `extension/`  | VS Code extension, TypeScript|
| WASM bridge   | `lang/`       | wasm-bindgen, wasm-pack      |

Owner: **@otabekoff** (Otabek Sadiridinov). Repo: `github.com/otabekoff/zink`. Branch: `main`.

## Architecture

```
Source (.zink) → Lexer (lexer.rs) → Tokens → Parser (parser.rs) → AST → Interpreter (interpreter.rs) → Output
```

- **Lexer** tokenizes source into `Token` enum variants (keywords, operators, literals, identifiers).
- **Parser** produces an AST of `Stmt` and `Expr` enums. Entry point: `parse()` returns `Vec<Stmt>`.
- **Interpreter** walks the AST. Values are the `Value` enum: `Number(f64)`, `Str(String)`, `Bool(bool)`, `Array(Vec<Value>)`, `Function(...)`, `Null`.
- **WASM**: `run_zink(code: &str) -> String` in `lib.rs` captures `say` output and returns it. Built with `wasm-pack build --target web`.

## Language Rules

- 5 types: number, string, boolean, array, function (+ null)
- 28 builtins — see `call_builtin()` in interpreter.rs
- String interpolation with `{expr}` inside double quotes — resolved at string creation time (in `eval_expr`), NOT at `say` time
- `say` prints; `loop N times { }` for counted loops; `while cond { }` for conditional loops
- `fn name(args) { }` for named functions; `fn(args) { }` for anonymous/lambda
- `let` for variable declaration; `=` for assignment; no `const`/`var`
- No semicolons, no type annotations (v0.1)
- Arrays are mutable; `reverse(arr)` and `sort(arr)` mutate in-place and return the array
- `type()` returns lowercase strings: `"number"`, `"string"`, `"boolean"`, `"array"`, `"function"`, `"null"`
- Empty arrays `[]` are falsy; non-empty arrays are truthy
- Comments start with `#`

## Coding Conventions

### Rust (lang/)
- Edition 2024, no external dependencies (except wasm-bindgen/js-sys for WASM target)
- No `#![allow(dead_code)]` — remove dead code instead
- No `_` prefix on public methods — rename them properly
- Run `cargo clippy` and `cargo test` before committing
- All examples in `lang/examples/` must pass: `cargo run -- examples/FILE.zink`
- Keep interpreter.rs `call_builtin()` match arms alphabetically ordered
- Use `Value::is_truthy()` for all truthiness checks

### TypeScript / React (playground/)
- React 19 + Vite 8 + Bun
- Single component: `ZinkIDE.tsx` — the entire playground
- WASM loaded from `/zink/zink_lang_bg.wasm` (base path `/zink/`)
- ESLint for linting: `bun run lint`

### Docs (docs/)
- VitePress 2 + Bun
- Base: `/zink/`
- Guide pages in `docs/guide/`, reference pages in `docs/reference/`
- Every builtin must be documented with examples
- Mark mutating builtins with a "Mutates" tag

### Extension (extension/)
- TextMate grammar in `syntaxes/zink.tmLanguage.json`
- Snippets in `snippets/zink.json`
- 5 commands: Run File, Run Selection, Open REPL, Open Playground, Open Docs
- VSIX packaged via `vsce package`

## Version Sync

Version must match in ALL these locations:
1. `lang/Cargo.toml` → `version`
2. `playground/package.json` → `version`
3. `docs/package.json` → `version`
4. `lang/src/main.rs` → REPL banner `"v{VERSION}"`
5. `playground/src/ZinkIDE.tsx` → footer version
6. `CHANGELOG.md` → latest entry header
7. `README.md` → Extension badge

## Git & Release Workflow

- Commit format: `type(scope): message` — e.g. `feat(lang): add for-in loops`
- Scopes: `lang`, `playground`, `docs`, `extension`, `ci`, `repo`
- Branch: always `main` (no feature branches for now)
- Tags: `vX.Y.Z` format — triggers `release.yml` workflow
- Release: update all versions → commit → tag → push tag → GitHub Release auto-creates
- WASM must be pre-built and committed to `playground/public/` before deployment

## CI/CD Pipelines

| Workflow       | File              | Triggers On              |
|----------------|-------------------|--------------------------|
| CI — Lang      | `ci-lang.yml`     | `lang/**` changes        |
| CI — Playground| `ci-ide.yml`      | `playground/**` changes  |
| CI — Docs      | `ci-docs.yml`     | `docs/**` changes        |
| Deploy         | `deploy.yml`      | push to main             |
| Release        | `release.yml`     | `v*` tags                |

## Common Mistakes to Avoid

1. **String interpolation at say-time** — interpolation happens when strings are CREATED (in `eval_expr` StringLiteral), not when `say` prints them
2. **Forgetting WASM rebuild** — after any lang/ change, WASM must be rebuilt and committed to playground/public/
3. **Version mismatch** — all 7 locations must be updated together
4. **Dead code** — don't leave unused functions; remove them or make them public
5. **Builtin count drift** — docs say "28 builtins" — keep this accurate when adding/removing
6. **Array mutability** — `reverse()` and `sort()` are in-place; they return the same array, not a copy
7. **Boolean type name** — `type(true)` returns `"boolean"`, not `"bool"`

## Key Files

| File                          | Purpose                                    |
|-------------------------------|---------------------------------------------|
| `lang/src/lexer.rs`          | Tokenizer — `Token` enum + `Lexer` struct   |
| `lang/src/parser.rs`         | Parser — `Stmt`/`Expr` enums + `Parser`     |
| `lang/src/interpreter.rs`    | Tree-walk interpreter + all builtins         |
| `lang/src/main.rs`           | CLI + REPL entry point                       |
| `lang/src/lib.rs`            | WASM entry — `run_zink()` function           |
| `playground/src/ZinkIDE.tsx`  | Entire playground UI (single component)     |
| `docs/.vitepress/config.mts` | VitePress config (nav, sidebar, base)        |
| `extension/package.json`     | Extension manifest (commands, grammar, etc.) |
| `DEVELOPMENT.md`             | Full developer guide & workflow              |
| `ROADMAP.md`                 | v0.1 → v1.0 detailed plan                   |

## Current State (v0.1.0)

- Tree-walk interpreter, 28 builtins, 5 types, string interpolation, higher-order functions
- 9 example programs in `lang/examples/` — all must pass
- Browser playground via WASM
- Full docs site
- VS Code extension with syntax + snippets + commands
- CI/CD fully operational
- See ROADMAP.md for what's next
