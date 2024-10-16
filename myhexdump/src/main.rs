use std::fs;
use std::io::Read;
use std::io::Seek;
use std::os::unix::fs::MetadataExt;
use std::process::exit;

use clap::Parser;

const BUFFER_SIZE: usize = 4096;

/// Simple HexDump Utility
#[derive(Parser)]
#[command(version)]
struct Args {
    /// File To Parse
    filename: String,

    /// interpret only length bytes of input
    #[arg(short = 'n', long, default_value = "0")]
    length: Option<u64>,

    /// offset bytes from the beginning
    #[arg(short = 's', long = "skip", default_value = "0")]
    offset: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let filename = args.filename;

    let length: u64 = args.length.unwrap_or_default();
    let offset: u64 = args.offset.unwrap_or_default();

    println!("HexDump File    [{}]", filename);
    println!("length:         [{}]", length);
    println!("offset:         [{}]", offset);

    let mut file = match fs::File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR: Failed to open file [{}], reason: {}", &filename, e);
            exit(1);
        }
    };

    let meta = match file.metadata() {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!(
                "ERROR: Failed to read metadata for file [{}], reason: {}",
                &filename, e
            );
            exit(1);
        }
    };

    let size = meta.size();
    println!("File Size:      [{}] bytes", size);

    if offset != 0 {
        if offset > size {
            eprintln!(
                "ERROR: Skipped[{}] past the end of file size[{}]",
                offset, size
            );
            exit(1);
        }
        match file.seek(std::io::SeekFrom::Start(offset)) {
            Ok(x) => x,
            Err(e) => {
                eprintln!(
                    "ERROR: Failed to skip [{}] bytes into file, reason: {}",
                    offset, e
                );
                exit(1);
            }
        };
    }

    let mut buffer: Vec<u8> = vec![0; BUFFER_SIZE];

    let mut bytes_remaining = size - offset;
    if length != 0 && length != bytes_remaining {
        bytes_remaining = length;
    }
    let mut printed_offset: u64 = offset;
    let mut bytes_printed: u64 = 0;

    while bytes_remaining > 0 {
        let bytes_read = match file.read(&mut buffer[..]) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("ERROR: Failed to read from file ({})", e);
                exit(1);
            }
        };
        let mut col_count: u8 = 0;
        let mut new_line: bool = true;
        let mut ascii_out: String = String::new();
        for item in buffer.iter().take(bytes_read) {
            if new_line {
                print!("{:08x} | ", printed_offset);
                new_line = false;
            }
            print!("{:02x} ", item);
            if *item >= 32 && *item <= 126 {
                ascii_out.push(*item as char);
            } else {
                ascii_out.push('.');
            }
            bytes_printed += 1;
            col_count += 1;
            if col_count == 8 {
                print!(" ");
            }
            if col_count >= 16 {
                println!(" |{ascii_out}|");
                new_line = true;
                col_count = 0;
                printed_offset += 16;
                ascii_out.clear();
            }
            if bytes_printed >= length {
                // reached n bytes to dump
                break;
            }
        }
        if col_count != 0 {
            // we ended with a partial row, print out the ascii text
            let padding = (16 - col_count) * 3;
            let pad_str = " ".to_string().repeat(padding.into());
            print!("{}", pad_str);
            if col_count < 8 {
                // add extra space to account for extra space in the middle of the line
                print!(" ");
            }

            println!(" |{ascii_out}|");
        }

        if bytes_remaining > bytes_read as u64 {
            bytes_remaining -= bytes_read as u64;
        } else {
            bytes_remaining = 0;
        }
    }
}
