use clap::Parser;

/// Simple sample program
#[derive(Parser)]
#[command(version)]
struct Args {
    /// name to parse
    name: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("Hello,          {}", args.name);
    println!("Verbose mode:   {:?}", args.verbose);
}
