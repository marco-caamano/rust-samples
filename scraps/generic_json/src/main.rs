use std::fs;

fn main() {
    println!("Hello, world!");

    // read filename from commandline argument
    let filename = std::env::args().nth(1).expect("missing filename argument");

    // open file in read-only mode
    let my_code = fs::read_to_string(filename).expect("cannot read file");

    // read json data
    let data: serde_json::Value = serde_json::from_str(&my_code).expect("cannot parse json");

    // print json data
    println!("{:?}", data);
}
