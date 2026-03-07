# Error Handling

Zink provides clear, line-numbered error messages at every stage of execution.

## Error Types

### Lex Errors

Occur when the source contains invalid characters or malformed tokens:

```
[Lex Error, line 3] Unterminated string
[Lex Error, line 7] Unknown char: '@'
```

### Parse Errors

Occur when the code structure is invalid:

```
[Parse Error, line 5] Expected '{' after if condition
[Parse Error, line 8] Expected ')' after arguments
```

### Runtime Errors

Occur during execution:

```
[Runtime Error, line 4] Undefined variable 'x'
[Runtime Error, line 6] Cannot call non-function value
[Runtime Error, line 9] Index out of bounds: 10
```

## Common Mistakes

### Missing braces

```zink
# Wrong — braces are required
if x > 0
  say "positive"

# Right
if x > 0 {
  say "positive"
}
```

### Using undeclared variables

```zink
# Wrong — forgot 'let'
x = 42

# Right
let x = 42
```

### Division by zero

```zink
say 10 / 0   # Runtime Error: Division by zero
```

## Tips

- Error messages always include the line number — check that line first
- The REPL catches errors without crashing, so you can fix and retry
- Use `type(value)` to check what type a value is at runtime

```zink
say type(42)       # number
say type("hi")     # string
say type(true)     # boolean
say type(null)     # null
say type([1,2])    # array
```
