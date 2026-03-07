# Operators

## Arithmetic

| Operator | Description    | Example     | Result |
|----------|----------------|-------------|--------|
| `+`      | Addition       | `3 + 4`     | `7`    |
| `-`      | Subtraction    | `10 - 3`    | `7`    |
| `*`      | Multiplication | `6 * 7`     | `42`   |
| `/`      | Division       | `10 / 3`    | `3.33` |
| `%`      | Modulo         | `10 % 3`    | `1`    |
| `-`      | Negation       | `-5`        | `-5`   |

`+` also concatenates strings:

```zink
say "hello" + " " + "world"   # hello world
```

## Comparison

| Operator | Description      | Example    | Result  |
|----------|------------------|------------|---------|
| `==`     | Equal            | `5 == 5`   | `true`  |
| `!=`     | Not equal        | `5 != 3`   | `true`  |
| `<`      | Less than        | `3 < 5`    | `true`  |
| `>`      | Greater than     | `5 > 3`    | `true`  |
| `<=`     | Less or equal    | `5 <= 5`   | `true`  |
| `>=`     | Greater or equal | `3 >= 5`   | `false` |

Comparison works on numbers and strings (lexicographic for strings).

## Logical

| Operator | Description | Example              | Result  |
|----------|-------------|----------------------|---------|
| `and`    | Logical AND | `true and false`     | `false` |
| `or`     | Logical OR  | `false or true`      | `true`  |
| `not`    | Logical NOT | `not true`           | `false` |

## Assignment

| Operator | Description | Example     |
|----------|-------------|-------------|
| `=`      | Assign      | `x = 42`    |

There are no compound assignment operators (`+=`, `-=`, etc.) in v0.1.0.

## Precedence

From highest to lowest:

1. Unary: `not`, `-` (negation)
2. Multiplicative: `*`, `/`, `%`
3. Additive: `+`, `-`
4. Comparison: `<`, `>`, `<=`, `>=`
5. Equality: `==`, `!=`
6. Logical AND: `and`
7. Logical OR: `or`
8. Assignment: `=`

Parentheses override precedence:

```zink
say (1 + 2) * 3   # 9
say 1 + 2 * 3     # 7
```
