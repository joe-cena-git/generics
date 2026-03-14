# Rust Generics — The Book, Chapter 10.1

Code examples from [**The Rust Programming Language**, Chapter 10.1: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html).

This project traces the progression from duplicated concrete code through to idiomatic generic Rust — covering generic functions, structs, enums, and methods.

## Output

```
The largest number is 100
The largest number is 100
The largest number is 6000
The largest number is 100
The largest number is 6000
The largest number is 100
The largest char is y
p.x = 5
p3.x = 5, p3.y = c
```

## Examples walkthrough

| Listing | Description |
|---------|-------------|
| 10-1 | Find the largest number in a single list using a manual loop |
| 10-2 | Duplicate the loop for a second list — the motivation for extraction |
| 10-3 | Extract a `largest(list: &[i32]) -> &i32` function to remove duplication |
| 10-4 | Two separate functions `largest_i32` / `largest_char` — duplication returns with different types |
| 10-5 | First attempt at `largest<T>` — does **not** compile; `T` needs a `PartialOrd` bound (shown commented-out) |
| 10-6 | `Point<T>` struct where both `x` and `y` share the same generic type |
| 10-7 | Why `Point { x: 5, y: 4.0 }` fails when `T` must be a single type (shown commented-out) |
| 10-8 | `Point<T, U>` struct with two independent type parameters for `x` and `y` |
| — | `Option<T>` and `Result<T, E>` from the standard library shown as enum examples |
| 10-9 | `impl<T> Point<T>` — a generic method `x()` returning `&T` |
| 10-10 | `impl Point<f32>` — a method `distance_from_origin` defined **only** for the concrete `f32` specialisation |
| 10-11 | `mixup<X2, Y2>` — a method with its own generic parameters distinct from the struct's, producing a new `Point<X1, Y2>` |

## Key concepts

- **Monomorphisation** — Rust generates concrete types at compile time, so generics have zero runtime cost.
- **Type parameter naming** — single uppercase letters (`T`, `U`, `X1`, `Y1`) are conventional.
- **Trait bounds** — `<T: PartialOrd>` (or `where T: PartialOrd`) are required before the compiler allows operators like `>` on generic types.
- **`impl` specialisation** — `impl Point<f32>` restricts methods to a single concrete instantiation of the generic struct.
- **Method-level generics** — a method can introduce its own type parameters independent of those on the struct.

## Running

```bash
cargo run
```

## Code navigation with Code Organizer

This project uses the [**Code Organizer**](https://marketplace.visualstudio.com/items?itemName=ran-codes.code-organizer) VS Code extension to navigate the listings directly from the sidebar.

![Code Organizer sidebar showing all 11 listings](assets/code-organizer.png)

### Syntax

Add a comment ending in `----` (four dashes) to create a navigable section. Nesting depth is controlled by the number of leading `//` characters:

```rust
// Top-level section ----
    // Second-level section ----
        //// Third-level section ----
```

Every section marker in `main.rs` follows this pattern, for example:

```rust
// 10-1 Finding the largest number in a list of numbers ----
// 10-6 A Point<T> struct that holds x and y values of type T ----
```

> The line **must** end with `----` to be recognised by Code Organizer.

## Reference

- [The Rust Programming Language — Chapter 10.1: Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Code Organizer — VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=ran-codes.code-organizer)
