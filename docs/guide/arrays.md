# Arrays

Arrays are ordered, dynamic-length lists that can hold any mix of types.

## Creating Arrays

```zink
let nums = [1, 2, 3, 4, 5]
let mixed = ["hello", 42, true, null]
let empty = []
```

## Accessing Elements

Zero-indexed with bracket notation:

```zink
let fruits = ["apple", "banana", "cherry"]
say fruits[0]   # apple
say fruits[2]   # cherry
```

## Modifying Arrays

```zink
let arr = [1, 2, 3]

# Change an element
arr[0] = 10
say arr   # [10, 2, 3]

# Add to the end
push(arr, 4)
say arr   # [10, 2, 3, 4]

# Remove from the end
let last = pop(arr)
say last  # 4
say arr   # [10, 2, 3]
```

## Array Length

```zink
let arr = [1, 2, 3]
say len(arr)   # 3
```

## Iterating

```zink
let colors = ["red", "green", "blue"]
let i = 0
while i < len(colors) {
  say "{i + 1}. {colors[i]}"
  i = i + 1
}
```

## Array Functions

| Function                   | Description                              |
|----------------------------|------------------------------------------|
| `len(arr)`                 | Number of elements                       |
| `push(arr, value)`         | Add to end                               |
| `pop(arr)`                 | Remove and return last element           |
| `reverse(arr)`             | Reverse in place (mutates)               |
| `sort(arr)`                | Sort in place (mutates)                  |
| `slice(arr, start, end)`   | Return sub-array                         |
| `contains(arr, value)`     | Check if value exists                    |
| `join(arr, separator)`     | Join elements into a string              |
| `map(arr, fn)`             | Transform each element                   |
| `filter(arr, fn)`          | Keep elements matching predicate         |
| `reduce(arr, fn, initial)` | Fold array into single value             |

## Examples

```zink
let nums = [5, 2, 8, 1, 9, 3]

# Sort
sort(nums)
say nums   # [1, 2, 3, 5, 8, 9]

# Map + Filter
fn square(x) { return x * x }
fn is_big(x) { return x > 10 }

let squared = map(nums, square)
let big = filter(squared, is_big)
say big   # [25, 64, 81]
```
