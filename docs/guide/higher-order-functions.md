# Higher-Order Functions

Zink treats functions as first-class values. You can pass them as arguments, return them from other functions, and store them in variables.

## map

Transform every element in an array:

```zink
fn double(x) { return x * 2 }

let nums = [1, 2, 3, 4, 5]
let result = map(nums, double)
say result   # [2, 4, 6, 8, 10]
```

## filter

Keep only elements that match a condition:

```zink
fn is_even(x) { return x % 2 == 0 }

let nums = [1, 2, 3, 4, 5, 6]
let evens = filter(nums, is_even)
say evens   # [2, 4, 6]
```

## reduce

Combine all elements into a single value:

```zink
fn add(a, b) { return a + b }

let nums = [1, 2, 3, 4, 5]
let total = reduce(nums, add, 0)
say total   # 15
```

## Chaining

Compose operations by nesting calls:

```zink
fn double(x) { return x * 2 }
fn is_big(x) { return x > 5 }
fn add(a, b) { return a + b }

let nums = [1, 2, 3, 4, 5]

# Double everything, keep values > 5, sum them
let result = reduce(filter(map(nums, double), is_big), add, 0)
say result   # 24 (6 + 8 + 10)
```

## Anonymous Functions

Pass inline functions without naming them:

```zink
let nums = [1, 2, 3, 4, 5]

let squared = map(nums, fn(x) { return x * x })
say squared   # [1, 4, 9, 16, 25]

let total = reduce(nums, fn(a, b) { return a + b }, 0)
say total   # 15
```

## Closures

Returned functions remember their enclosing scope:

```zink
fn multiplier(factor) {
  fn multiply(x) {
    return x * factor
  }
  return multiply
}

let triple = multiplier(3)
say triple(5)    # 15
say triple(10)   # 30
```
