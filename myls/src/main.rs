use colorsys::Rgb;
use devicons::{icon_for_file, Theme};
use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

#[derive(Debug)]
struct FileItem {
    path: String,
    filename: String,
    extension: String,
    is_file: bool,
    is_dir: bool,
    is_symlink: bool,
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

fn parse_dir_entry(entry: std::fs::DirEntry) -> Result<FileItem, String> {
    let my_path = match entry.path().to_str() {
        Some(path) => path.to_string(),
        None => {
            println!("ERROR: failed to get convert path");
            exit(1);
        }
    };
    let my_filename = match entry.file_name().into_string() {
        Ok(name) => name,
        Err(e) => {
            println!("ERROR: failed to convert filename {:?}", e);
            exit(1);
        }
    };
    let my_extension = match entry.path().extension() {
        Some(ext) => match ext.to_str() {
            Some(my_ext) => my_ext.to_string(),
            None => {
                println!("ERROR: Failed to convert string");
                exit(1);
            }
        },
        None => "".to_owned(),
    };
    let my_filetype = match entry.file_type() {
        Ok(data) => data,
        Err(e) => {
            println!(
                "ERROR: failed to get the filetype for [{}] ({})",
                my_filename, e
            );
            exit(1);
        }
    };
    let icon = icon_for_file(Path::new(&my_path), Some(Theme::Dark));
    let my_icon = if my_filetype.is_dir() {
        ''
    } else {
        icon.icon
    };

    let my_color = if my_filetype.is_dir() {
        "#3483eb".to_string()
    } else {
        icon.color.to_string()
    };

    let item = FileItem {
        path: my_path,
        filename: my_filename,
        extension: my_extension,
        is_file: my_filetype.is_file(),
        is_dir: my_filetype.is_dir(),
        is_symlink: my_filetype.is_symlink(),
        color: my_color,
        icon: my_icon,
    };

    Ok(item)
}

fn parse_single_file(path: &Path) -> Result<FileItem, String> {
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

    let my_extension = match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(x) => x.to_string(),
            None => "".to_string(),
        },
        None => "".to_string(),
    };

    let icon = icon_for_file(Path::new(&my_path), Some(Theme::Dark));
    let my_icon = if path.is_dir() { '' } else { icon.icon };

    let my_color = if path.is_dir() {
        "#3483eb".to_string()
    } else {
        icon.color.to_string()
    };

    let item = FileItem {
        path: my_path,
        filename: my_filename,
        extension: my_extension,
        is_file: path.is_file(),
        is_dir: path.is_dir(),
        is_symlink: path.is_symlink(),
        color: my_color,
        icon: my_icon,
    };

    Ok(item)
}

/*
* TODO
* - create enum for file types (file, dir, symlink, ...)
* - reorg files
*
*/

fn main() {
    let mut my_files: Vec<FileItem> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let target_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // we dont have a parameter so take the current directory
        get_current_path()
    };

    let path = Path::new(&target_path);

    if path.is_dir() {
        let data = match fs::read_dir(&path) {
            Ok(x) => x,
            Err(e) => {
                println!("ERROR: Failed to read directory ({:?})->({:?})", &path, e);
                exit(1);
            }
        };

        data.for_each(|item| {
            let dir_entry = item.unwrap_or_else(|e| {
                println!("Got Empty data for entry {}", e);
                exit(1);
            });
            let item = match parse_dir_entry(dir_entry) {
                Ok(data) => data,
                Err(e) => {
                    println!("ERROR: Failed to parse dir_entry[{}]", e);
                    exit(1);
                }
            };
            my_files.push(item);
        });
    } else {
        let item = match parse_single_file(path) {
            Ok(data) => data,
            Err(e) => {
                println!("ERROR: Failed to parse file ({})", e);
                exit(1);
            }
        };
        my_files.push(item);
    }

    for item in &my_files {
        println!(
            "{}{} {}{}",
            start_color(&item.color),
            item.icon,
            item.filename,
            stop_color()
        );
    }
}
