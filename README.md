Python-style collection comprehensions in Rust
==============================================

The structure is consistent with Python's [list and dictionary comprehensions](https://docs.python.org/3/reference/expressions.html#displays-for-lists-sets-and-dictionaries).

- If the loop body (everything before the first `for`) contains two
expressions separated by `:`, it will be treated as a hash map comprehension.

- If the loop body ends with a `;`, it will be a statement comprehension
(it won't evaluate to a collection).

- Otherwise it's a vector comprehension.

Comprehensions consist of a body followed by a `for ... in ...` expression,
followed by any combination of `for ... in ...` or `if ...` expressions.

The `for ... in ...` and `if ...` expressions are nested left-to-right, so
```rust
c!(do_something(x, y); for x in 0..10 if x % 2 == 1 for y in 'a'..='z');
```

is equivalent to:

```rust
for x in 0..10 {
    if x % 2 == 1 {
        for y in 'a'..='z' {
            do_something(x, y);
        }
    }
}
```

## Examples

### Vectors

- A simple vector comprehension:
```rust
let v = c![x * (1 + x) for x in 1..=10];
println!("{:?}", v);
```

```
[2, 6, 12, 20, 30, 42, 56, 72, 90, 110]
```

- Multiple iterators and conditionals:
```rust
let v = c![x * y for x in 1..=10 if x % 2 != 0 for y in -2..=2 if x > y];
println!("{:?}", v);
```
```
[-2, -1, 0, -6, -3, 0, 3, 6, -10, -5, 0, 5, 10, -14, -7, 0, 7, 14, -18, -9, 0, 9, 18]
```

### Hash Maps

- A simple hash map comprehension:
```rust
let m = c!{x: x * (1 + x) for x in 1..=10};
println!("{:?}", m);
```
```
{2: 6, 5: 30, 8: 72, 4: 20, 3: 12, 6: 42, 1: 2, 7: 56, 9: 90, 10: 110}
```

- Complex hash map comprehension:
```rust
let m = c!{format!("[{}|{}]", x, y): y.to_string().repeat(x) for x in 1..=3 for y in 'A'..='C'};
println!("{:?}", m);
```
```
{"[3|C]": "CCC", "[3|B]": "BBB", "[3|A]": "AAA", "[2|B]": "BB", "[1|C]": "C", "[1|A]": "A", "[1|B]": "B", "[2|C]": "CC", "[2|A]": "AA"}
```

### Statements

- A simple statement comprehension:
```rust
let mut n = 1;
c!(n *= x; for x in 1..=10);
println!("{}", n);
```
```
3628800
```
