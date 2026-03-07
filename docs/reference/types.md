# Types

Zink is dynamically typed. Values carry their type at runtime, and variables can hold any type.

## Number

64-bit IEEE 754 floating point. Integers and decimals are both numbers.

```zink
let x = 42
let pi = 3.14159
let negative = -10
```

All arithmetic produces numbers:

```zink
say 10 / 3   # 3.3333333333333335
say 7 % 2    # 1
```

## String

Double-quoted text with interpolation and escape support.

```zink
let s = "Hello, World!"
let interp = "2 + 2 = {2 + 2}"
let escaped = "line one\nline two"
```

Strings are immutable — operations return new strings.

## Boolean

`true` or `false`.

```zink
let yes = true
let no = false
say yes and no   # false
say yes or no    # true
say not yes      # false
```

## Null

Represents the absence of a value.

```zink
let nothing = null
say nothing   # null
say type(nothing)  # null
```

## Array

Ordered, mutable, dynamic-length collection. Can hold mixed types.

```zink
let arr = [1, "two", true, null, [5, 6]]
say arr[0]     # 1
say arr[4][1]  # 6
say len(arr)   # 5
```

Arrays are passed by reference — mutations are visible to the caller:

```zink
fn add_item(arr, item) {
  push(arr, item)
}

let list = [1, 2]
add_item(list, 3)
say list   # [1, 2, 3]
```

## Function

Functions are values. They can be stored in variables, passed as arguments, and returned from other functions.

```zink
fn greet(name) {
  say "Hello, {name}!"
}

say type(greet)   # function
```

## Type Checking

Use `type(value)` to get the type name as a string:

| Value       | `type()` result |
|-------------|-----------------|
| `42`        | `"number"`      |
| `"hello"`   | `"string"`      |
| `true`      | `"boolean"`     |
| `null`      | `"null"`        |
| `[1, 2]`    | `"array"`       |
| `fn(x){}`   | `"function"`    |
