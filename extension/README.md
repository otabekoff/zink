# Zink Language — VS Code Extension

Full-featured [Zink](https://github.com/nicbarker/zink) language support for Visual Studio Code — syntax highlighting, snippets, run/debug commands, REPL integration, and configurable settings.

![Zink](images/icon.png)

## Features

### Editor

- **Syntax highlighting** — Full TextMate grammar covering keywords, strings with `{interpolation}`, comments, numbers, operators, built-in functions, and more
- **Snippets** — Quick templates for `fn`, `if`, `while`, `loop`, `let`, `say`, `map`, `filter`, `reduce`
- **Bracket matching** — Auto-closing and matching for `{}`, `[]`, `()`, `""`
- **Indentation** — Automatic indent/outdent for blocks
- **Folding** — Collapse code blocks
- **Comment toggling** — `Ctrl+/` toggles `#` line comments
- **File icon** — `.zink` files display the Zink lightning-bolt icon

### Commands

| Command                | Keybinding               | Description                              |
| ---------------------- | ------------------------ | ---------------------------------------- |
| **Zink: Run File**     | `Ctrl+Shift+R`           | Save and run the active `.zink` file     |
| **Zink: Run Selection** | `Ctrl+Shift+Enter`      | Pipe selected text (or current line) to the interpreter |
| **Zink: Open REPL**   | `Ctrl+Shift+Z`           | Open an interactive Zink REPL terminal   |
| **Zink: Restart REPL** | —                       | Dispose the current REPL and start fresh |
| **Zink: Build Interpreter** | —                  | Run `cargo build --release` in the `lang/` directory |

Commands are also available from the editor title bar (▶ play button) and the right-click context menu when a `.zink` file is open.

### Settings

| Setting                        | Default | Description                                          |
| ------------------------------ | ------- | ---------------------------------------------------- |
| `zink.interpreterPath`         | `""`    | Custom path to the `zink` binary. Empty = use PATH.  |
| `zink.runInIntegratedTerminal` | `true`  | Run in integrated terminal (`true`) or output channel |
| `zink.clearTerminalBeforeRun`  | `true`  | Clear the terminal before each run                   |

## Installation

### From VSIX (local)

```bash
cd extension
npm install
npm run compile
npm run package
code --install-extension zink-lang-0.1.0.vsix
```

### From source (development)

1. Open the `extension/` folder in VS Code
2. Run `npm install`
3. Press `F5` to launch the Extension Development Host
4. Open any `.zink` file to see everything in action

## Requirements

- **VS Code** ≥ 1.75.0
- **Zink interpreter** — install from source (`cargo install --path lang`) or place the `zink` binary on your `PATH`

## File Associations

The extension associates `.zink` files with the Zink language mode.

## Example

```zink
# Fibonacci sequence
fn fib(n) {
  if n <= 1 { return n }
  return fib(n - 1) + fib(n - 2)
}

loop 10 times {
  say "fib({i}) = {fib(i)}"
}
```

## License

MIT — see [LICENSE](../LICENSE)
