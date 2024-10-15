fn colored(r: i32, g: i32, b: i32, text: &str) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text)
}

fn main() {
    println!("Hello, console colors!");
    println!("{}", colored(255, 0, 0, "Testing Red"));
    println!("{}", colored(0, 255, 0, "Testing Blue"));
    println!("{}", colored(0, 0, 255, "Testing Green"));
    println!("{}", colored(103, 46, 232, "Testing Foo"));
}
