# Fluent Input 🦀

Simple, fluent input for Rust CLI apps.

---

## Usage

```rust
use input::input;

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