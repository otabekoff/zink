# Variables

Variables in Zink are declared with `let`. They are dynamically typed — no type annotations needed.

## Declaration

```zink
let x = 42
let name = "Alice"
let active = true
let nothing = null
```

## Reassignment

Variables can be reassigned to any type:

```zink
let x = 10
say x        # 10

x = "hello"
say x        # hello
```

## Types

Zink has five value types:

| Type    | Example            | Notes                        |
|---------|--------------------|------------------------------|
| Number  | `42`, `3.14`       | 64-bit floating point        |
| String  | `"hello"`          | Double-quoted, with interp.  |
| Boolean | `true`, `false`    | Logical values               |
| Null    | `null`             | Absence of a value           |
| Array   | `[1, 2, 3]`        | Ordered, dynamic-length      |

## Naming Rules

- Must start with a letter or underscore
- Can contain letters, digits, and underscores
- Case-sensitive (`name` ≠ `Name`)

```zink
let my_var = 1       # ok
let _private = 2     # ok
let camelCase = 3    # ok
```

## Scope

Variables declared inside a block (`{ }`) are local to that block:

```zink
let x = "outer"
if true {
  let x = "inner"
  say x              # inner
}
say x                # outer
```
