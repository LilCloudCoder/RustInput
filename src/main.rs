use RustInput::input;

fn main() {
    let age: i32 = input("Enter age: ").int();
    println!("Age: {}", age);
}