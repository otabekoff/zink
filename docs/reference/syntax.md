# Syntax Reference

## Comments

```zink
# This is a comment
say "hello"  # Inline comment
```

Comments start with `#` and continue to end of line.

## Variables

```zink
let name = value
name = new_value
```

## Functions

```zink
fn name(param1, param2) {
  # body
  return value
}
```

Anonymous functions:

```zink
fn(x) { return x * 2 }
```

## Print

```zink
say expression
say "interpolated: {expr}"
```

## If / Else

```zink
if condition {
  # ...
} else if condition {
  # ...
} else {
  # ...
}
```

## While Loop

```zink
while condition {
  # ...
}
```

## Loop N Times

```zink
loop count times {
  # ...
}
```

## Arrays

```zink
let arr = [1, 2, 3]
arr[0]              # access
arr[0] = 10         # assign
```

## Strings

```zink
"plain string"
"interpolation: {expr}"
"escape: \n \t \\ \""
```

## Operators (Precedence, high to low)

| Precedence | Operators            | Description       |
|------------|----------------------|-------------------|
| 1          | `not`, `-` (unary)   | Unary             |
| 2          | `*`, `/`, `%`        | Multiplicative    |
| 3          | `+`, `-`             | Additive          |
| 4          | `<`, `>`, `<=`, `>=` | Comparison        |
| 5          | `==`, `!=`           | Equality          |
| 6          | `and`                | Logical AND       |
| 7          | `or`                 | Logical OR        |

## Keywords

```
let  fn  return  if  else  while  loop  times  say
and  or  not  true  false  null
```

## Semicolons

Optional. Zink uses newlines and braces to determine statement boundaries.
