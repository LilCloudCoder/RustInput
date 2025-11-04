# Fluent Input 🦀

Simple, fluent input for Rust CLI apps — easier to use than the standard `std::io`.

---

## Install

Add to your project's `Cargo.toml` (example):

```toml
[dependencies]
rustinput = { git = "https://github.com/lilcloudcoder/RustInput" }
```

Note: inside this repository the crate name is `RustInput` (PascalCase). In external projects, Cargo will expose it as `rustinput` (snake_case).

## Quick start

```rust
// In external projects
use rustinput::input; // or: use rustinput::Input;

fn main() {
    let a: i32 = input("Enter i32: ").int();
    let b: f64 = input("Enter f64: ").float();
    let name = input("Enter name: ").string();
    let agree = input("Agree? ").bool();

    println!("{a} {b} {name} {agree}");
}
```

Inside this repo's examples/binary use:

```rust
use RustInput::input;
```

## Features
- Ergonomic prompts: `input("Your age: ").int::<i32>()`
- All integer and float types supported via `FromStr`
- Strings and booleans with common aliases (`y/yes/true/1`, `n/no/false/0`)
- New helpers:
  - `char()` — read a single character
  - `optional::<T>() -> Option<T>` — Enter to skip
  - `default::<T>(value) -> T` — Enter to accept default
  - `choices(&["light", "dark"]) -> String` — restrict to allowed values (case-insensitive)

## API cheatsheet

```rust
use rustinput::input;

let i: i64 = input("i64: ").int();
let u: usize = input("usize: ").int();
let f: f32 = input("f32: ").float();
let s: String = input("name: ").string();
let b: bool = input("ok? ").bool();
let c: char = input("initial: ").char();
let maybe_n: Option<i32> = input("number (Enter to skip): ").optional();
let tries: u32 = input("tries (default 3): ").default(3);
let theme: String = input("theme [light/dark]: ").choices(&["light", "dark"]);
```

### Types supported
- Integers: all signed/unsigned (`i8..i128`, `u8..u128`, `isize`, `usize`)
- Floats: `f32`, `f64`
- Bool, String, Char