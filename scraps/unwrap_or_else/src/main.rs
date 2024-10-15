use std::env;
use std::process::exit;

fn main() {
    println!("Hello, unwrap or else!");
    if env::args().len() == 1 {
        eprintln!("ERROR need at least one argument foo|bar");
        exit(1);
    }
    let args: Vec<String> = env::args().collect();
    let value = args[1].as_str();
    let result = guess_who(value).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        return 999999;
    });
    println!("Got Result [{}]", result);

    let result2 = guess_who(value);
    if result2.is_err() {
        eprintln!("ERROR: {}", result2.unwrap_err());
        exit(1);
    }
    println!("Got Result2 [{}]", result2.unwrap()); // this will not panic as we previoysly check
                                                    // for error, but it doesn't flow right....guess will stick with match
}

fn guess_who(val: &str) -> Result<u32, String> {
    match val {
        "foo" => Ok(46),
        "bar" => Ok(678),
        _ => Err("Invalid Argument".to_string()),
    }
}
