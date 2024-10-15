fn main() {
    println!("Hello, Sample Ranges!");

    println!("---------------");

    for i in 0..5 {
        println!("i[{i}]");
    }
    println!("---------------");

    for i in 0..=5 {
        println!("i[{i}]");
    }
    println!("---------------");

    for i in 3..=5 {
        println!("i[{i}]");
    }
    println!("---------------");

    let text = "Hello World";
    println!("text -> [{}]", text);

    println!("---------------");
    println!("text -> [{}]", &text[0..3]);

    println!("---------------");
    println!("text -> [{}]", &text[..3]);

    println!("---------------");
    println!("text -> [{}]", &text[0..=3]);

    println!("---------------");
    println!("text -> [{}]", &text[3..=6]);

    println!("\n\n");
}
