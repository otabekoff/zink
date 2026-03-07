# Strings

Strings in Zink are double-quoted and support interpolation.

## Basics

```zink
let greeting = "Hello, World!"
say greeting   # Hello, World!
say len(greeting)  # 13
```

## String Interpolation

Embed expressions inside `{` and `}` within strings:

```zink
let name = "Alice"
let age = 30

say "Name: {name}"
say "Age: {age}"
say "Next year: {age + 1}"
say "2 + 2 = {2 + 2}"
```

## Escape Sequences

| Escape | Output    |
|--------|-----------|
| `\n`   | Newline   |
| `\t`   | Tab       |
| `\\`   | Backslash |
| `\"`   | Quote     |

```zink
say "Line one\nLine two"
say "She said \"hello\""
```

## String Functions

| Function             | Description                    |
|----------------------|--------------------------------|
| `len(s)`             | String length                  |
| `upper(s)`           | Convert to uppercase           |
| `lower(s)`           | Convert to lowercase           |
| `trim(s)`            | Remove leading/trailing spaces |
| `split(s, sep)`      | Split into array               |
| `contains(s, sub)`   | Check if substring exists      |
| `slice(s, start, end)` | Extract substring            |

## Examples

```zink
let s = "  Hello, World!  "

say trim(s)         # Hello, World!
say upper("hello")  # HELLO
say lower("HELLO")  # hello

let parts = split("a,b,c", ",")
say parts           # [a, b, c]

say contains("hello world", "world")  # true
say slice("hello", 1, 3)              # el
```

## Concatenation

Use `+` to join strings:

```zink
let first = "Hello"
let second = "World"
say first + ", " + second + "!"   # Hello, World!
```
