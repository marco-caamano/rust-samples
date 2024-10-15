use colorsys::{ColorAlpha, Hsl, Rgb};

fn main() {
    println!("Hello, color conversion!");

    //let color1 = Rgb::from("#ff00ff");
    let color1 = Rgb::from_hex_str("#f950f0").unwrap();
    println!("color1 {:?}", color1);
    println!(
        "R[{}] G[{}] B[{}]",
        color1.red(),
        color1.green(),
        color1.blue(),
    );
}
