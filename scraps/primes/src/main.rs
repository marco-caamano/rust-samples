use clap::Parser;

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
}

fn main() {
    let args = Args::parse();
    let mut start: u32 = args.start;
    let mut end: u32 = args.end;

    // sanity checks
    if end < start {
        println!("ERROR: End is smaller than start, assuming inverted range");
        std::mem::swap(&mut start, &mut end);
    }
    if start == 0 {
        println!("ERROR: Invalid Start at 0, assuming 1 instead");
        start = 1;
    }

    println!("Start           {}", start);
    println!("End             {}", end);
    println!("Verbose mode:   {:?}", args.verbose);

    let result: Vec<u32> = calculate_primes(start, end, args.verbose);
    println!("Result: {:?}", result);
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

fn calculate_primes(start: u32, end: u32, verbose: bool) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for i in start..=end {
        if verbose {
            println!("Testing i {}", i);
        }
        if is_prime(i) {
            result.push(i);
            if verbose {
                println!("i {} is prime", i);
            }
        } else if verbose {
            println!("i {} is NOT prime", i);
        }
    }

    result
}
