use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

/// Calculate Prime Numbers from a range
#[derive(Parser)]
#[command(version)]
struct Args {
    /// start at number
    start: u32,

    /// end at number
    end: u32,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Write stats to output file
    #[arg(short, long)]
    output: Option<String>,
}

struct Context {
    start: u32,
    end: u32,
    verbose: bool,
    has_output: bool,
    filename: String,
}

impl Context {
    fn new() -> Context {
        Context {
            start: 0,
            end: 0,
            verbose: false,
            has_output: false,
            filename: String::new(),
        }
    }
}

#[derive(Debug)]
struct Item {
    number: u32,
    is_prime: bool,
    duration: std::time::Duration,
}

fn main() {
    let mut ctx = Context::new();
    let args = Args::parse();
    ctx.start = args.start;
    ctx.end = args.end;
    ctx.verbose = args.verbose;

    if Option::is_some(&args.output) {
        ctx.has_output = true;
        ctx.filename = args.output.unwrap();
    }

    // sanity checks
    if ctx.end < ctx.start {
        println!("ERROR: End is smaller than start, assuming inverted range");
        std::mem::swap(&mut ctx.start, &mut ctx.end);
    }
    if ctx.start == 0 {
        println!("ERROR: Invalid Start at 0, assuming 1 instead");
        ctx.start = 1;
    }
    if ctx.has_output && Path::new(&ctx.filename).exists() {
        println!(
            "ERROR: Output file [{}] already exists, will not overwrite file, aborting",
            ctx.filename
        );
        exit(1);
    }

    println!("Start           {}", ctx.start);
    println!("End             {}", ctx.end);
    println!("Verbose mode:   {:?}", ctx.verbose);
    if ctx.has_output {
        println!("Output file:    {}", &ctx.filename);
    }

    let start = Instant::now();
    let results: Vec<Item> = calculate_primes(&ctx);
    let duration = start.elapsed();
    println!("Primes:");
    for item in results.iter() {
        if item.is_prime {
            if ctx.verbose {
                println!("Number: {} duration: {:?} ", item.number, item.duration);
            } else {
                print!("{}, ", item.number);
            }
        }
    }
    if !ctx.verbose {
        println!();
    }
    println!(
        "Execution Took: [{}] seconds | [{}] milliseconds",
        duration.as_secs(),
        duration.as_millis()
    );

    if ctx.has_output {
        let path = Path::new(&ctx.filename);
        let mut file: File = match File::create(path) {
            Ok(file) => file,
            Err(e) => {
                println!(
                    "ERROR: Failed to open output file [{}] reason: {}",
                    &ctx.filename, e,
                );
                exit(1);
            }
        };
        for item in results.iter() {
            if item.is_prime {
                match writeln!(file, "{},{}", &item.number, &item.duration.as_nanos()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("ERROR: Write failed [{}], aborting", e);
                        exit(1);
                    }
                }
            }
        }
    }
}

fn is_prime(num: u32) -> bool {
    // num is prime only if it is divisible only by 1 and the same number
    if num == 1 {
        // special case exit early
        return true;
    }
    // test
    for i in 2..num {
        if (num % i) == 0 {
            // num is divisible by i. Num is not prime
            return false;
        }
    }
    // if we didn't find an invalid case then this is a prime number
    true
}

fn calculate_primes(ctx: &Context) -> Vec<Item> {
    let mut result: Vec<Item> = Vec::new();
    for num in ctx.start..=ctx.end {
        if ctx.verbose {
            println!("Testing num {}", num);
        }
        let start = Instant::now();
        let num_is_prime = is_prime(num);
        let item = Item {
            number: num,
            is_prime: num_is_prime,
            duration: start.elapsed(),
        };
        result.push(item);
        if ctx.verbose && num_is_prime {
            println!("Number [{}] is prime", num);
        } else if ctx.verbose {
            println!("Number [{}] is NOT prime", num);
        }
    }

    result
}
