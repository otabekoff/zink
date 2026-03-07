# Zink Playground

Browser-based IDE for the [Zink](https://github.com/otabekoff/zink) programming language. Write, run, and explore Zink code with syntax highlighting, examples, and integrated docs.

## Features

- Syntax highlighting with single-pass regex (keywords, strings, numbers, functions, comments)
- 8 runnable examples with descriptions
- Documentation sidebar with quick reference
- Line numbers, execution timer, error display
- Keyboard shortcuts: `Ctrl+Enter` to run, `Tab` for indentation

## Stack

- **React 19** + **TypeScript 5.9**
- **Vite 8** (build + dev server)
- **ESLint** with typescript-eslint

## Development

```bash
npm install
npm run dev          # dev server at localhost:5173
npm run build        # production build (tsc + vite)
npm run lint         # lint
npm run preview      # preview production build
```

## Architecture

```
src/
├── main.tsx               # Entry point
├── ZinkIDE.tsx            # Main IDE component (editor, sidebar, output)
├── zink-interpreter.js    # Full Zink interpreter (JS port)
├── zink-interpreter.d.ts  # Type declarations for the interpreter
└── index.css              # Base styles
```

The interpreter is kept as a plain `.js` file to avoid TypeScript strict-mode issues with the dynamically-typed interpreter classes. A `.d.ts` file provides the type interface.

## Deployment

Deployed to GitHub Pages via the `ci-ide.yml` workflow on push to `main`.

## License

MIT — see [LICENSE](../LICENSE)
