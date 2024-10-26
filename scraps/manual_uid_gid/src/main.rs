use std::collections::HashMap;
use std::fs;
use std::process::exit;
use std::str::FromStr;

fn map_ids(path: &str) -> HashMap<u32, String> {
    let mut map: HashMap<u32, String> = HashMap::new();

    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read /etc/passwd file. Reason: {}", e);
            exit(1);
        }
    };

    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(':').collect();
        if cols.len() < 3 {
            // something wrong with the line we got
            continue;
        }
        let username = cols[0].to_string();
        let id = match FromStr::from_str(cols[2]) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to covert uid[{}] to number. Reason: {}", cols[2], e);
                continue;
            }
        };
        map.insert(id, username);
    }

    map
}

fn main() {
    println!("Manual UID GID mapping from /etc/passwd and /etc/group files");
    let password_map = map_ids("/etc/passwd");
    println!("/etc/passwd Parsed:");
    println!("+----------+----------------------+");
    for (uid, username) in password_map {
        println!("| {:>8} | {:<20} |", uid, username);
    }
    println!("+----------+----------------------+");

    println!();

    let group_map = map_ids("/etc/group");
    println!("/etc/group Parsed:");
    println!("+----------+----------------------+");
    for (gid, group) in group_map {
        println!("| {:>8} | {:<20} |", gid, group);
    }
    println!("+----------+----------------------+");
}
