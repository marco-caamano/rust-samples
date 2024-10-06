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

fn start_color(color: &str) -> String {
    let color = match Rgb::from_hex_str(color) {
        Ok(color) => color,
        Err(e) => {
            eprintln!("ERROR: failed to parse color {}. Error: {}", color, e);
            Rgb::new(255.0, 255.0, 255.0, Some(255.0))
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
            eprintln!("ERROR: failed to get current directory [{}]", e);
            exit(1);
        }
    };
    match path.to_str() {
        Some(cwd) => cwd.to_owned(),
        None => {
            eprintln!("ERROR: found invalid characters in current path {:?}", path);
            exit(1);
        }
    }
}

fn parse_file_entry(path: &String) -> Result<FileItem, String> {
    let my_path = Path::new(path);

    let os_filename = match my_path.file_name() {
        Some(name) => name,
        None => return Err("Failed to parse filename".to_string()),
    };

    let my_filename = match os_filename.to_owned().into_string() {
        Ok(fname) => fname,
        Err(_) => return Err("Failed to parse filename".to_string()),
    };

    let my_icon: char;
    let file_type: FileType;
    let my_color: String;
    if my_path.is_dir() {
        file_type = FileType::Dir;
        my_icon = '';
        my_color = "#3483eb".to_string();
    } else if my_path.is_file() {
        file_type = FileType::File;
        let icon = icon_for_file(Path::new(&my_path), Some(Theme::Dark));
        my_icon = icon.icon;
        my_color = icon.color.to_string();
    } else {
        // TODO: review the color or any other special handling for symlink
        file_type = FileType::Symlink;
        let icon = icon_for_file(Path::new(&my_path), Some(Theme::Dark));
        my_icon = icon.icon;
        my_color = icon.color.to_string();
    }

    let item = FileItem {
        path: path.to_string(),
        filename: my_filename,
        filetype: file_type,
        color: my_color,
        icon: my_icon,
    };

    Ok(item)
}

fn parse_directory(path: &Path, items: &mut Vec<String>) {
    let data = match fs::read_dir(path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("ERROR: Failed to read directory ({:?})->({:?})", path, e);
            // return empty vec
            return;
        }
    };

    data.for_each(|item| {
        match item {
            Ok(entry) => {
                let entry_path = entry.path().to_owned();
                if let Some(path) = entry_path.to_str() {
                    items.push(path.to_string());
                }
            }
            Err(e) => {
                eprintln!("Error reading directory entry {}", e);
            }
        };
    });
}

fn parse_path(target_path: &String) -> Vec<FileItem> {
    let mut my_paths: Vec<String> = Vec::new();
    let mut my_files: Vec<FileItem> = Vec::new();

    let path = Path::new(target_path);
    if path.is_dir() {
        parse_directory(path, &mut my_paths);
    } else {
        my_paths.push(target_path.to_string());
    }

    for path in my_paths.iter() {
        let item = match parse_file_entry(path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("ERROR: Failed to parse file ({})", e);
                exit(1);
            }
        };
        my_files.push(item);
    }

    my_files
}

fn main() {
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
            let path = Path::new(&arg);
            if path.exists() {
                paths_to_parse.push(arg);
            } else {
                eprintln!("Cannot access [{}] (path does not exist)", arg);
            }
        }
    }
    if paths_to_parse.is_empty() {
        // since we did not get any paths to parse default to current directory
        paths_to_parse.push(get_current_path());
    }

    let mut add_path_separator = false;
    for target_path in paths_to_parse.iter() {
        let my_files: Vec<FileItem> = parse_path(target_path);

        // Output the contents
        if paths_to_parse.len() > 1 {
            // we have multiple paths so prepend dir
            if add_path_separator {
                println!();
            }
            println!("{}:", target_path);
            add_path_separator = true;
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
    }
}
