# Zink Documentation

Documentation site for the [Zink](https://github.com/otabekoff/zink) programming language, built with [VitePress](https://vitepress.dev/).

## Development

```bash
bun install
bun run dev          # dev server at localhost:5173
bun run build        # production build
bun run preview      # preview production build
```

## Structure

```
guide/
├── what-is-zink.md          # Overview
├── getting-started.md       # Installation + first program
├── variables.md             # let bindings, types, scope
├── functions.md             # fn, return, closures
├── control-flow.md          # if/else, while, loop
├── arrays.md                # Array creation, methods
├── strings.md               # Interpolation, escapes, methods
├── higher-order-functions.md # map, filter, reduce
└── error-handling.md        # Error types and debugging

reference/
├── syntax.md                # Complete syntax reference
├── builtins.md              # All built-in functions
├── operators.md             # Operator table + precedence
└── types.md                 # Type system reference
```

## License

MIT — see [LICENSE](https://github.com/otabekoff/zink/blob/main/LICENSE)
