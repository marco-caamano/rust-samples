use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn get_current_path() -> String {
    match env::current_dir() {
        Ok(path) => path
            .to_str()
            .unwrap_or_else(|| {
                println!("Failed to read read path (Invalid Characters found)");
                exit(1);
            })
            .to_string(),
        Err(e) => {
            println!("ERROR: failed to get current directory [{}]", e);
            exit(1);
        }
    }
}

/*
* TODO
* - create struct to represent each item
* - create enum for file types (file, dir, symlink, ...)
* - extract file extension using path().extension
* - add parsing of LC_COLORS to assign file color based on extension, store it in hashmap
* - reorg files
*
*/

fn main() {
    let cwd = get_current_path();
    println!("Current Path [{cwd}]");
    let path = Path::new(&cwd);
    if !path.is_dir() {
        println!("ERROR: Path is not a directory");
        exit(1);
    }
    let data = fs::read_dir(path).unwrap_or_else(|e| {
        println!("ERROR: Failed to read directory ({})->({})", &cwd, e);
        exit(1);
    });
    data.for_each(|item| {
        let dir_entry = item.unwrap_or_else(|e| {
            println!("Got Empty data for entry {}", e);
            exit(1);
        });
        println!(
            "Data path {:?} | filename [{:?}] | file_type [{:?}]",
            dir_entry.path(),
            dir_entry.file_name(),
            dir_entry.file_type()
        );
    });
    //for entry in fs::read_dir(path) {
    //    let entry = entry;
    //}
}
