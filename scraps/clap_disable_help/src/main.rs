use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None, disable_help_flag=true)]
struct Args {
    /// name to parse
    name: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Human Readable output
    #[arg(short, long)]
    human: bool,
}

fn main() {
    let args = Args::parse();
    println!("Hello,          {}", args.name);
    println!("Verbose mode:   {:?}", args.verbose);
    println!("Human Readable: {:?}", args.human);
}
