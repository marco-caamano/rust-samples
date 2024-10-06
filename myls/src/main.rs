use colorsys::Rgb;
use devicons::{icon_for_file, Theme};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::FileType;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::process::exit;
use std::usize;
use users::{Groups, Users, UsersCache};

#[derive(Debug)]
struct FileItem {
    filename: String,
    filetype: FileType,
    filesize: u64,
    num_links: u64,
    uid: u32,
    gid: u32,
    color: String,
    icon: char,
}

struct ListingFlags {
    show_all: bool,
    show_details: bool,
}

impl ListingFlags {
    fn new() -> ListingFlags {
        ListingFlags {
            show_all: false,
            show_details: false,
        }
    }
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

    let metadata = match my_path.metadata() {
        Ok(data) => data,
        Err(_) => {
            return Err("Failed to get metadata".to_string());
        }
    };

    let os_filename = match my_path.file_name() {
        Some(name) => name,
        None => return Err("Failed to parse filename".to_string()),
    };

    let my_filename = match os_filename.to_owned().into_string() {
        Ok(fname) => fname,
        Err(_) => return Err("Failed to parse filename".to_string()),
    };

    let my_icon: char;
    let my_color: String;
    if my_path.is_dir() {
        my_icon = '';
        my_color = "#3483eb".to_string();
    } else if my_path.is_file() {
        let icon = icon_for_file(Path::new(&my_path), Some(Theme::Dark));
        my_icon = icon.icon;
        my_color = icon.color.to_string();
    } else {
        // TODO: review the color or any other special handling for symlink
        let icon = icon_for_file(Path::new(&my_path), Some(Theme::Dark));
        my_icon = icon.icon;
        my_color = icon.color.to_string();
    }

    let item = FileItem {
        filename: my_filename,
        filetype: metadata.file_type(),
        filesize: metadata.len(),
        num_links: metadata.nlink(),
        uid: metadata.uid(),
        gid: metadata.gid(),
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

fn simple_listing(items: &Vec<FileItem>, flags: &ListingFlags) {
    for item in items {
        if item.filename.starts_with(".") && !flags.show_all {
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

fn get_num_width(val: u64) -> usize {
    let mut width = 0;
    let mut my_val = val;
    loop {
        width += 1;
        my_val /= 10;
        if my_val == 0 {
            break;
        }
    }
    width
}

fn detailed_listing(items: &Vec<FileItem>, flags: &ListingFlags) {
    // first traverse the filelist to get max widths
    let mut max_size_width: usize = 0;
    let mut max_link_width: usize = 0;
    for item in items {
        let size_width = get_num_width(item.filesize);
        if size_width > max_size_width {
            max_size_width = size_width;
        }
        let size_link = get_num_width(item.num_links);
        if size_link > max_link_width {
            max_link_width = size_link;
        }
    }
    let mut cache = UsersCache::new();
    for item in items {
        if item.filename.starts_with(".") && !flags.show_all {
            // skip hidden file
            continue;
        }
        // start with filetype
        let ftype = item.filetype;
        if ftype.is_dir() {
            print!("d");
        } else if ftype.is_symlink() {
            print!("l");
        } else {
            print!("-");
        }
        // TODO list proper permissions
        print!("--------- ");

        let my_links = item.num_links;
        print!("{my_links:>max_link_width$} ");

        let os_user = match cache.get_user_by_uid(item.uid) {
            Some(user) => user.name().to_owned(),
            None => OsString::new(),
        };

        let user = match os_user.into_string() {
            Ok(user) => user,
            Err(_) => {
                eprintln!("ERROR: failed to convert user");
                String::new()
            }
        };

        let os_group = match cache.get_group_by_gid(item.gid) {
            Some(group) => group.name().to_owned(),
            None => OsString::new(),
        };

        let group = match os_group.into_string() {
            Ok(group) => group,
            Err(_) => {
                eprintln!("ERROR: failed to convert group");
                String::new()
            }
        };

        // TODO need to add padding to user/group see ls -l /tmp for
        // caamao and root owned files
        print!("{} {} ", user, group);

        let my_size = item.filesize;
        print!("{my_size:>max_size_width$} ");

        print!("[date] ");

        println!(
            "{}{} {}{}",
            start_color(&item.color),
            item.icon,
            item.filename,
            stop_color()
        );
    }
}

/*
* TODO
* - Parse LS_COLORS, extract colors for dir and symlink from env
*   and convert to RGB for text coloring
*
*/

fn main() {
    let mut paths_to_parse: Vec<String> = Vec::new();
    let mut flags: ListingFlags = ListingFlags::new();

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
                flags.show_all = true;
            }
            if arg.contains('l') {
                flags.show_details = true;
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
        if flags.show_details {
            detailed_listing(&my_files, &flags);
        } else {
            simple_listing(&my_files, &flags);
        }
    }
}
