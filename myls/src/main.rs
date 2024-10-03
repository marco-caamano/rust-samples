use colorsys::Rgb;
use devicons::{icon_for_file, Theme};
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

#[derive(Debug)]
enum FileType {
    File,
    Dir,
    Symlink,
}

#[derive(Debug)]
struct FileItem {
    path: String,
    filename: String,
    filetype: FileType,
    color: String,
    icon: char,
}

fn start_color(color: &String) -> String {
    let color = match Rgb::from_hex_str(color) {
        Ok(color) => color,
        Err(e) => {
            println!("ERROR: failed to parse color ({})", e);
            exit(1);
        }
    };

    format!(
        "\x1B[38;2;{};{};{}m",
        color.red() as u8,
        color.green() as u8,
        color.blue() as u8
    )
}

fn stop_color() -> String {
    "\x1B[0m".to_string()
}

fn get_current_path() -> String {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            println!("ERROR: failed to get current directory [{}]", e);
            exit(1);
        }
    };
    match path.to_str() {
        Some(cwd) => cwd.to_owned(),
        None => {
            println!("ERROR: found invalid characters in path");
            exit(1);
        }
    }
}

fn parse_file_entry(path: &Path) -> Result<FileItem, String> {
    let my_path = match path.to_str() {
        Some(data) => data.to_string(),
        None => return Err("Failed to parse path".to_string()),
    };

    let my_filename = match path.file_name() {
        Some(name) => match name.to_owned().into_string() {
            Ok(fname) => fname,
            Err(_) => return Err("Failed to parse filename".to_string()),
        },
        None => return Err("Failed to parse filename".to_string()),
    };

    let icon = icon_for_file(Path::new(&my_path), Some(Theme::Dark));
    let my_icon = if path.is_dir() { '' } else { icon.icon };

    let my_color = if path.is_dir() {
        "#3483eb".to_string()
    } else {
        icon.color.to_string()
    };

    let file_type: FileType;
    if path.is_file() {
        file_type = FileType::File;
    } else if path.is_dir() {
        file_type = FileType::Dir;
    } else {
        file_type = FileType::Symlink;
    }

    let item = FileItem {
        path: my_path,
        filename: my_filename,
        filetype: file_type,
        color: my_color,
        icon: my_icon,
    };

    Ok(item)
}

fn main() {
    let mut my_files: Vec<FileItem> = Vec::new();
    let mut paths_to_parse: Vec<String> = Vec::new();
    let mut all_files: bool = false;

    let mut skipped_first: bool = false;
    for arg in env::args() {
        if !skipped_first {
            // skip the first argument that is the program name
            skipped_first = true;
            continue;
        }
        if arg.starts_with('-') {
            // we have a group of command line parameters
            if arg.contains('a') {
                all_files = true;
            }
        } else {
            // then this is a path to parse
            paths_to_parse.push(arg);
        }
    }
    if paths_to_parse.is_empty() {
        // since we did not get any paths to parse default to current directory
        paths_to_parse.push(get_current_path());
    }

    for target_path in paths_to_parse.iter() {
        let path = Path::new(target_path);
        if path.is_dir() {
            let data = match fs::read_dir(path) {
                Ok(x) => x,
                Err(e) => {
                    println!("ERROR: Failed to read directory ({:?})->({:?})", path, e);
                    exit(1);
                }
            };

            data.for_each(|item| {
                let dir_entry = item.unwrap_or_else(|e| {
                    println!("Got Empty data for entry {}", e);
                    exit(1);
                });

                let entry_path = dir_entry.path();
                let my_path = entry_path.as_path();
                let item = match parse_file_entry(my_path) {
                    Ok(data) => data,
                    Err(e) => {
                        println!("ERROR: Failed to parse dir_entry[{}]", e);
                        exit(1);
                    }
                };
                my_files.push(item);
            });
        } else {
            let item = match parse_file_entry(path) {
                Ok(data) => data,
                Err(e) => {
                    println!("ERROR: Failed to parse file ({})", e);
                    exit(1);
                }
            };
            my_files.push(item);
        }

        // Output the contents
        if paths_to_parse.len() > 1 {
            // we have multiple paths so prepend dir
            println!("\n{}:", target_path);
        }
        for item in &my_files {
            if item.filename.starts_with(".") && !all_files {
                // skip hidden file
                continue;
            }
            println!(
                "{}{} {}{}",
                start_color(&item.color),
                item.icon,
                item.filename,
                stop_color()
            );
        }
        my_files.clear();
    }
}
