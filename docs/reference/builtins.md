# Built-in Functions

Zink ships with 30+ built-in functions. No imports needed.

## I/O

| Function   | Description            | Example                |
|------------|------------------------|------------------------|
| `say`      | Print to output        | `say "hello"`          |

> `say` is a keyword, not a function — it doesn't use parentheses.

## Type Conversion

| Function   | Description              | Example               |
|------------|--------------------------|------------------------|
| `str(v)`   | Convert to string        | `str(42)` → `"42"`    |
| `num(s)`   | Convert to number        | `num("42")` → `42`    |
| `type(v)`  | Get type name as string  | `type(42)` → `"number"` |

## Math

| Function          | Description               | Example                |
|-------------------|---------------------------|------------------------|
| `floor(n)`        | Round down                | `floor(3.7)` → `3`    |
| `ceil(n)`         | Round up                  | `ceil(3.2)` → `4`     |
| `round(n)`        | Round to nearest          | `round(3.5)` → `4`    |
| `abs(n)`          | Absolute value            | `abs(-5)` → `5`       |
| `sqrt(n)`         | Square root               | `sqrt(16)` → `4`      |
| `pow(base, exp)`  | Exponentiation            | `pow(2, 8)` → `256`   |
| `max(a, b)`       | Larger of two             | `max(3, 7)` → `7`     |
| `min(a, b)`       | Smaller of two            | `min(3, 7)` → `3`     |
| `random()`        | Random float 0..1         | `random()` → `0.4217` |

## Array Functions

| Function                    | Description                       | Example                              |
|-----------------------------|-----------------------------------|--------------------------------------|
| `len(arr)`                  | Number of elements                | `len([1,2,3])` → `3`                |
| `push(arr, value)`          | Append to end (mutates)           | `push(arr, 4)`                       |
| `pop(arr)`                  | Remove and return last            | `pop(arr)` → last element            |
| `reverse(arr)`              | Reverse in place                  | `reverse([1,2,3])` → `[3,2,1]`      |
| `sort(arr)`                 | Sort in place                     | `sort([3,1,2])` → `[1,2,3]`         |
| `slice(arr, start, end)`    | Sub-array (non-mutating)          | `slice([1,2,3,4], 1, 3)` → `[2,3]`  |
| `contains(arr, value)`      | Check membership                  | `contains([1,2,3], 2)` → `true`     |
| `join(arr, separator)`      | Join into string                  | `join(["a","b"], ",")` → `"a,b"`    |
| `range(start, end, step?)`  | Generate number array             | `range(0, 5, 1)` → `[0,1,2,3,4]`   |

## String Functions

| Function              | Description                  | Example                           |
|-----------------------|------------------------------|-----------------------------------|
| `len(s)`              | String length                | `len("hi")` → `2`                |
| `upper(s)`            | To uppercase                 | `upper("hi")` → `"HI"`           |
| `lower(s)`            | To lowercase                 | `lower("HI")` → `"hi"`           |
| `trim(s)`             | Strip whitespace             | `trim("  hi  ")` → `"hi"`        |
| `split(s, sep)`       | Split into array             | `split("a,b", ",")` → `["a","b"]`|
| `contains(s, sub)`    | Check substring              | `contains("abc", "bc")` → `true` |
| `slice(s, start, end)`| Extract substring            | `slice("hello", 0, 2)` → `"he"` |

## Higher-Order Functions

| Function                     | Description                          | Example                              |
|------------------------------|--------------------------------------|--------------------------------------|
| `map(arr, fn)`               | Transform each element               | `map([1,2,3], double)` → `[2,4,6]`  |
| `filter(arr, fn)`            | Keep matching elements               | `filter([1,2,3,4], is_even)` → `[2,4]` |
| `reduce(arr, fn, initial)`   | Fold into single value               | `reduce([1,2,3], add, 0)` → `6`     |
