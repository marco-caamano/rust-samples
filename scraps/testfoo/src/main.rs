fn main() {
    println!("Hello, test foo!");

    let number: Option<i32> = Some(15);

    if let Some(x) = number {
        println!("The number is {}", x);
    } else {
        println!("No number found");
        std::process::exit(1);
    }

    let val = number.unwrap();
    println!("val {:?}", val);
}
