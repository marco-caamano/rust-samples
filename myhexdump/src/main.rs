use std::fs;
use std::io::Seek;
use std::process::exit;

use clap::Parser;

/// Simple HexDump Utility
#[derive(Parser)]
#[command(version)]
struct Args {
    /// File To Parse
    filename: String,

    /// interpret only length bytes of input
    #[arg(short = 'n', long, default_value = "0")]
    length: Option<u64>,

    /// skip offset bytes from the beginning
    #[arg(short, long, default_value = "0")]
    skip: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let filename = args.filename;

    let length: u64 = args.length.unwrap_or_default();
    let skip: u64 = args.skip.unwrap_or_default();

    println!("HexDump File    [{}]", filename);
    println!("length:         [{}]", length);
    println!("skip:           [{}]", skip);

    let mut file = match fs::File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR: Failed to open file [{}], reason: {}", &filename, e);
            exit(1);
        }
    };
    if skip != 0 {
        match file.seek(std::io::SeekFrom::Start(skip)) {
            Ok(x) => x,
            Err(e) => {
                eprintln!(
                    "ERROR: Failed to skip [{}] bytes into file, reason: {}",
                    skip, e
                );
                exit(1);
            }
        };
    }

    println!("Completed OK");
}
