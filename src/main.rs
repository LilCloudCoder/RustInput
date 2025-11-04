use RustInput::input;

fn main() {
    let age: i32 = input("Enter age: ").int();
    let height: f64 = input("Enter height (m): ").float();
    let name = input("Enter name: ").string();
    let confirm = input("Proceed? (y/n): ").bool();
    let initial = input("First initial: ").char();
    let lucky = input("Lucky number (press Enter to skip): ").optional::<i32>();
    let theme = input("Theme [light/dark]: ").choices(&["light", "dark"]);
    let retries = input("Retries [default 3]: ").default::<u32>(3);

    println!(
        "name={name}, age={age}, height={height}, confirm={confirm}, initial={initial}, lucky={lucky:?}, theme={theme}, retries={retries}"
    );
}