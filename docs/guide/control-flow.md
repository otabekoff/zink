# Control Flow

## If / Else

```zink
let x = 10

if x > 0 {
  say "positive"
} else if x < 0 {
  say "negative"
} else {
  say "zero"
}
```

Braces are **required**. There's no ternary operator.

## While Loop

```zink
let i = 0
while i < 5 {
  say "i = {i}"
  i = i + 1
}
```

## Loop N Times

A cleaner way to repeat something a fixed number of times:

```zink
loop 3 times {
  say "hello"
}
# hello
# hello
# hello
```

The count can be any expression:

```zink
let n = 5
loop n times {
  say "repeat"
}
```

## Comparison Operators

| Operator | Meaning          |
|----------|------------------|
| `==`     | Equal            |
| `!=`     | Not equal        |
| `<`      | Less than        |
| `>`      | Greater than     |
| `<=`     | Less or equal    |
| `>=`     | Greater or equal |

## Logical Operators

| Operator | Meaning     |
|----------|-------------|
| `and`    | Logical AND |
| `or`     | Logical OR  |
| `not`    | Logical NOT |

```zink
let age = 25
let has_id = true

if age >= 18 and has_id {
  say "access granted"
}
```

## Truthiness

Falsy values: `false`, `null`, `0`, `""` (empty string).

Everything else is truthy, including empty arrays.
