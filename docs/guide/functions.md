# Functions

Functions are first-class values in Zink. Define them with `fn`, call them by name.

## Basic Functions

```zink
fn greet(name) {
  say "Hello, {name}!"
}

greet("Alice")   # Hello, Alice!
greet("Bob")     # Hello, Bob!
```

## Return Values

Use `return` to send a value back from a function:

```zink
fn add(a, b) {
  return a + b
}

let result = add(3, 4)
say result   # 7
```

## Recursion

Functions can call themselves:

```zink
fn factorial(n) {
  if n <= 1 {
    return 1
  }
  return n * factorial(n - 1)
}

say factorial(5)   # 120
```

## Functions as Values

Functions are first-class — assign them to variables, pass them as arguments:

```zink
fn double(x) {
  return x * 2
}

let nums = [1, 2, 3, 4, 5]
let doubled = map(nums, double)
say doubled   # [2, 4, 6, 8, 10]
```

## Anonymous Functions

Inline function expressions for quick callbacks:

```zink
let nums = [1, 2, 3, 4, 5]
let evens = filter(nums, fn(x) { return x % 2 == 0 })
say evens   # [2, 4]
```

## Closures

Functions capture their surrounding scope:

```zink
fn make_counter() {
  let count = 0
  fn increment() {
    count = count + 1
    return count
  }
  return increment
}

let counter = make_counter()
say counter()   # 1
say counter()   # 2
say counter()   # 3
```

## Parameters

- Functions accept zero or more parameters
- Arguments are passed by value (numbers, strings, booleans) or by reference (arrays)
- Extra arguments are ignored; missing arguments default to `null`
