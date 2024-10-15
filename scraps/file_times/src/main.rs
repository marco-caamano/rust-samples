use chrono::{DateTime, Local};
use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn main() {
    println!("Hello, file times");

    if env::args().len() < 2 {
        eprintln!("ERROR need at least one parameter");
        std::process::exit(1);
    }
    let args: Vec<String> = env::args().collect();
    let target = args[1].clone();

    println!("Testing target [{target}]");

    let metadata = match fs::metadata(&target) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("ERROR failed to read metadata: {}", e);
            std::process::exit(1);
        }
    };

    let mtime = metadata.mtime();
    let mod_time = UNIX_EPOCH + std::time::Duration::from_secs(mtime as u64);
    println!(
        "mtime: {:?}  mod_time: {:?} | Parsed: {}",
        mtime,
        mod_time,
        parse_time(mod_time)
    );

    let ctime = metadata.ctime();
    let cre_time = UNIX_EPOCH + std::time::Duration::from_secs(ctime as u64);
    println!(
        "ctime: {:?}  cre_time: {:?} | Parsed: {}",
        ctime,
        cre_time,
        parse_time(cre_time)
    );

    let atime = metadata.atime();
    let acc_time = UNIX_EPOCH + std::time::Duration::from_secs(atime as u64);
    println!(
        "atime: {:?}  acc_time: {:?} | Parsed: {}",
        atime,
        acc_time,
        parse_time(acc_time)
    );

    println!("\n\n");

    let my_path = Path::new(&target);

    let sym_metadata = match my_path.symlink_metadata() {
        Ok(data) => data,
        Err(_) => {
            eprintln!("Failed to get syn metadata");
            std::process::exit(1);
        }
    };
    let sym_mtime = match sym_metadata.modified() {
        Ok(time) => time,
        Err(e) => {
            eprintln!("ERROR: failed to get sym metadata: {}", e);
            std::process::exit(1);
        }
    };
    println!("SYM mtime: {}", parse_time(sym_mtime));
}

fn parse_time(data: SystemTime) -> String {
    let ltime: DateTime<Local> = DateTime::from(data);
    ltime.format("%b %d %H:%M").to_string()
}
