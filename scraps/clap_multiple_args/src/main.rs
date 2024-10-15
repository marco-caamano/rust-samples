use clap::Parser;

/// Simple sample program
#[derive(Parser)]
#[command(version)]
struct Args {
    /// names to parse
    names: Vec<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("Names           {:?}", args.names);
    println!("Verbose mode:   {:?}", args.verbose);
}
