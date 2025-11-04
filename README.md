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

    println!("{} {} {} {}", a, b, name, agree);
}
```
### Types Supported:
- Integers: * (signed/unsigned)
- Floats: * (32/64)
- Bool/Str