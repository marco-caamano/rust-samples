use chrono::{DateTime, Datelike, Local};
use clap::Parser;
use devicons::{icon_for_file, Theme};
use libc::{
    S_IRGRP, S_IROTH, S_IRUSR, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP,
    S_IXOTH, S_IXUSR,
};
use std::collections::HashMap;
use std::env;
use std::fs::{self, Metadata};
use std::os::unix::fs::FileTypeExt;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::time::SystemTime;

const KB: f64 = 1024.0;
const MB: f64 = 1024.0 * KB;
const GB: f64 = 1024.0 * MB;
const TB: f64 = 1024.0 * GB;

const ERR_EMPTY_STRING: &str = "Empty Hex String";
const ERR_INVALID_FORMAT: &str = "Invalid format (must be #XXXXXX)";
const ERR_PARSE_ERROR: &str = "Failed Parsing Hex Value";

/// Simple LS implementation
#[derive(Parser)]
#[command(version, disable_help_flag = true)]
struct Args {
    /// PATHs to process. Can be a path or a single file path
    paths: Vec<String>,

    /// Show all hidden items
    #[arg(short, long)]
    all: bool,

    /// Show details
    #[arg(short = 'l', long = "listing")]
    details: bool,

    /// Sort by modification time
    #[arg(short, long)]
    time: bool,

    /// Reverse sorting
    #[arg(short, long)]
    reverse: bool,

    /// Human readable details
    #[arg(short, long)]
    human: bool,

    /// Show help
    #[arg(long)]
    help: bool,
}

#[derive(Debug)]
struct FileItem {
    filename: String,
    abs_metadata: Metadata,
    sym_metadata: Metadata,
    symlink_path: String,
    filesize: u64,
    human_readable_size: String,
    user: String,
    group: String,
    color: String,
    icon: char,
    modified: String,
    last_modified: SystemTime,
    mode: u32,
}

struct Context {
    uid_map: HashMap<u32, String>,
    gid_map: HashMap<u32, String>,
    flags: ListingFlags,
}

impl Context {
    fn new() -> Context {
        Context {
            uid_map: HashMap::new(),
            gid_map: HashMap::new(),
            flags: ListingFlags::new(),
        }
    }
}

struct ListingFlags {
    show_all: bool,
    show_details: bool,
    reverse_sort: bool,
    sort_by_time: bool,
    human_readable: bool,
}

impl ListingFlags {
    fn new() -> ListingFlags {
        ListingFlags {
            show_all: false,
            show_details: false,
            reverse_sort: false,
            sort_by_time: false,
            human_readable: false,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    fn new() -> Rgb {
        Rgb {
            red: 255,
            green: 255,
            blue: 255,
        }
    }
}

fn extract_rgb(value: &str) -> Result<Rgb, &str> {
    if value.is_empty() {
        return Err(ERR_EMPTY_STRING);
    }
    if value.len() != 7 || !value.starts_with('#') {
        return Err(ERR_INVALID_FORMAT);
    }

    let red_str = &value[1..3];
    let red = match u8::from_str_radix(red_str, 16) {
        Ok(number) => number,
        Err(_) => return Err(ERR_PARSE_ERROR),
    };

    let green_str = &value[3..5];
    let green = match u8::from_str_radix(green_str, 16) {
        Ok(number) => number,
        Err(_) => return Err(ERR_PARSE_ERROR),
    };

    let blue_str = &value[5..7];
    let blue = match u8::from_str_radix(blue_str, 16) {
        Ok(number) => number,
        Err(_) => return Err(ERR_PARSE_ERROR),
    };

    Ok(Rgb { red, green, blue })
}

fn map_ids(path: &str, map: &mut HashMap<u32, String>) {
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read {} file. Reason: {}", path, e);
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
}

fn start_color(color: &str) -> String {
    let color = match extract_rgb(color) {
        Ok(color) => color,
        Err(e) => {
            eprintln!("ERROR: failed to parse color {}. Error: {}", color, e);
            Rgb::new()
        }
    };

    format!("\x1B[38;2;{};{};{}m", color.red, color.green, color.blue)
}

fn stop_color() -> &'static str {
    "\x1B[0m"
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

fn parse_file_entry(path: &String, ctx: &Context) -> Result<FileItem, String> {
    let my_path = Path::new(path);

    // Use symlink_metadata to not traverse any symbolic my_links
    // (it seems it is named backwards...)
    let sym_metadata = match my_path.symlink_metadata() {
        Ok(data) => data,
        Err(_) => {
            return Err("Failed to get syn metadata".to_string());
        }
    };

    let abs_metadata = match my_path.metadata() {
        Ok(data) => data,
        Err(_) => {
            return Err("Failed to get abs metadata".to_string());
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
        my_color = "#3483eb".to_string()
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

    let user = match ctx.uid_map.get(&sym_metadata.uid()) {
        Some(user) => user.to_owned(),
        None => "----".to_owned(),
    };

    let group = match ctx.gid_map.get(&sym_metadata.gid()) {
        Some(group) => group.to_owned(),
        None => "----".to_owned(),
    };

    let last_modified = match sym_metadata.modified() {
        Ok(time) => time,
        Err(e) => {
            eprintln!("ERROR: Failed to read modified time for item: {}", e);
            SystemTime::UNIX_EPOCH
        }
    };

    let ltime: DateTime<Local> = DateTime::from(last_modified);
    let file_year = ltime.year();
    let current_time = Local::now();
    let current_year = current_time.year();
    let modified_time = if current_year != file_year {
        format!("{}", ltime.format("%b %e  %Y"))
    } else {
        format!("{}", ltime.format("%b %e %H:%M"))
    };

    let mode = sym_metadata.mode();

    let symlink_path: String = if sym_metadata.is_symlink() {
        let path = match fs::read_link(my_path) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to read path from symlink: {}", e);
                PathBuf::from("")
            }
        };
        let os_path = path.to_str().unwrap_or("");
        os_path.to_string()
    } else {
        String::new()
    };

    let item = FileItem {
        filename: my_filename,
        sym_metadata: sym_metadata.clone(),
        abs_metadata: abs_metadata.clone(),
        symlink_path,
        filesize: sym_metadata.len(),
        human_readable_size: get_human_readable(sym_metadata.len()),
        user,
        group,
        modified: modified_time,
        color: my_color,
        icon: my_icon,
        mode,
        last_modified,
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

fn parse_path(target_path: &String, ctx: &Context) -> Vec<FileItem> {
    let mut my_paths: Vec<String> = Vec::new();
    let mut my_files: Vec<FileItem> = Vec::new();

    let path = Path::new(target_path);
    if path.is_dir() {
        parse_directory(path, &mut my_paths);
    } else {
        my_paths.push(target_path.to_string());
    }

    for path in my_paths.iter() {
        let item = match parse_file_entry(path, ctx) {
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

fn parse_mode(mode: u32) -> String {
    let mut perm = String::new();

    if mode & S_IRUSR == S_IRUSR {
        perm.push('r');
    } else {
        perm.push('-');
    }
    if mode & S_IWUSR == S_IWUSR {
        perm.push('w');
    } else {
        perm.push('-');
    }
    if mode & S_IXUSR == S_IXUSR {
        // check for suid bit
        if mode & S_ISUID == S_ISUID {
            perm.push('s');
        } else {
            perm.push('x');
        }
    } else {
        // check for suid bit
        if mode & S_ISUID == S_ISUID {
            perm.push('S');
        } else {
            perm.push('-');
        }
    }

    if mode & S_IRGRP == S_IRGRP {
        perm.push('r');
    } else {
        perm.push('-');
    }
    if mode & S_IWGRP == S_IWGRP {
        perm.push('w');
    } else {
        perm.push('-');
    }
    if mode & S_IXGRP == S_IXGRP {
        // check for guid bit
        if mode & S_ISGID == S_ISGID {
            perm.push('s');
        } else {
            perm.push('x');
        }
    } else {
        // check for guid bit
        if mode & S_ISGID == S_ISGID {
            perm.push('S');
        } else {
            perm.push('-');
        }
    }

    if mode & S_IROTH == S_IROTH {
        perm.push('r');
    } else {
        perm.push('-');
    }
    if mode & S_IWOTH == S_IWOTH {
        perm.push('w');
    } else {
        perm.push('-');
    }
    if mode & S_IXOTH == S_IXOTH {
        // check for sticky bit
        if mode & S_ISVTX == S_ISVTX {
            perm.push('t');
        } else {
            perm.push('x');
        }
    } else {
        // check for sticky bit
        if mode & S_ISVTX == S_ISVTX {
            perm.push('T');
        } else {
            perm.push('-');
        }
    }

    perm
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

fn get_human_readable(my_size: u64) -> String {
    let val: f64 = my_size as f64;
    if val > TB {
        format!("{:.1}T", val / TB)
    } else if val > GB {
        format!("{:.1}G", val / GB)
    } else if val > MB {
        format!("{:.1}M", val / MB)
    } else if val > KB {
        format!("{:.1}K", val / KB)
    } else {
        format!("{my_size}")
    }
}

fn detailed_listing(items: &Vec<FileItem>, flags: &ListingFlags) {
    // first traverse the filelist to get max widths
    let mut max_size_width: usize = 0;
    let mut max_human_size_width: usize = 0;
    let mut max_link_width: usize = 0;
    let mut max_user_width: usize = 0;
    let mut max_group_width: usize = 0;
    for item in items {
        if item.filename.starts_with(".") && !flags.show_all {
            // skip hidden file
            continue;
        }
        if flags.human_readable {
            let human_width = item.human_readable_size.len();
            if human_width > max_human_size_width {
                max_human_size_width = human_width;
            }
        } else {
            let size_width = get_num_width(item.filesize);
            if size_width > max_size_width {
                max_size_width = size_width;
            }
        }
        let size_link = get_num_width(item.sym_metadata.nlink());
        if size_link > max_link_width {
            max_link_width = size_link;
        }
        let user_len = item.user.len();
        if user_len > max_user_width {
            max_user_width = user_len;
        }
        let group_len = item.group.len();
        if group_len > max_group_width {
            max_group_width = group_len;
        }
    }
    for item in items {
        if item.filename.starts_with(".") && !flags.show_all {
            // skip hidden file
            continue;
        }

        // start with filetype
        let ftype: char;
        let mode = item.mode;
        let mut is_symlink = false;

        if item.sym_metadata.is_dir() {
            ftype = 'd';
        } else if item.sym_metadata.is_symlink() {
            ftype = 'l';
            is_symlink = true;
        } else if item.abs_metadata.file_type().is_char_device() {
            ftype = 'c';
        } else if item.sym_metadata.file_type().is_block_device() {
            ftype = 'b';
        } else {
            ftype = '-';
        }
        let perms = parse_mode(mode);

        print!("{}{} ", ftype, perms);

        let my_links = item.sym_metadata.nlink();
        print!("{my_links:>max_link_width$} ");

        let user = &item.user;
        let group = &item.group;
        print!("{user:>max_user_width$} {group:>max_group_width$} ");

        if flags.human_readable {
            let my_human_size = &item.human_readable_size;
            print!("{my_human_size:>max_human_size_width$} ");
        } else {
            let my_size = item.filesize;
            print!("{my_size:>max_size_width$} ");
        }

        print!("{} ", item.modified);

        if is_symlink {
            // we need to handle colors a bit different
            print!(
                "{}{} {}{}",
                start_color("#09bfc9"),
                item.icon,
                item.filename,
                stop_color()
            );
            print!(
                " -> {}{} {}{}",
                start_color(&item.color),
                item.icon,
                item.symlink_path,
                stop_color()
            );
        } else {
            print!(
                "{}{} {}{}",
                start_color(&item.color),
                item.icon,
                item.filename,
                stop_color()
            );
        }

        println!();
    }
}

fn show_help() {
    println!("\nMyLS: very simple implementation of ls utility written in Rust\n");
    println!("Usage: myls [OPTION]... [FILE]...");
    println!("Arguments:");
    println!("    -a       Show hidden files");
    println!("    -l       Show long listing");
    println!("    -h       Show human readable sizes");
    println!("    -t       Sort by last modified time");
    println!("    -r       Reverse sort\n");
}

/*
* TODO
* - Parse LS_COLORS, extract colors for dir and symlink from env
*   and convert to RGB for text coloring
*
*/

fn main() {
    let mut paths_to_parse: Vec<String> = Vec::new();
    let mut ctx: Context = Context::new();

    let args = Args::parse();

    if args.all {
        ctx.flags.show_all = true;
    }
    if args.details {
        ctx.flags.show_details = true;
    }
    if args.time {
        ctx.flags.sort_by_time = true;
    }
    if args.human {
        ctx.flags.human_readable = true;
    }
    if args.reverse {
        ctx.flags.reverse_sort = true;
    }
    if args.help {
        show_help();
        exit(0);
    }
    for item in args.paths {
        paths_to_parse.push(item);
    }
    if paths_to_parse.is_empty() {
        // since we did not get any paths to parse default to current directory
        paths_to_parse.push(get_current_path());
    }

    // parse the uid and gid
    map_ids("/etc/passwd", &mut ctx.uid_map);
    map_ids("/etc/group", &mut ctx.gid_map);

    let mut add_path_separator = false;
    for target_path in paths_to_parse.iter() {
        let mut my_files: Vec<FileItem> = parse_path(target_path, &ctx);

        // Output the contents
        if paths_to_parse.len() > 1 {
            // we have multiple paths so prepend dir
            if add_path_separator {
                println!();
            }
            println!("{}:", target_path);
            add_path_separator = true;
        }

        // Sort files
        if ctx.flags.sort_by_time {
            // sort by modified time
            if ctx.flags.reverse_sort {
                // reverse order
                my_files.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
            } else {
                my_files.sort_by(|a, b| a.last_modified.cmp(&b.last_modified));
            }
        } else {
            // sort by filename
            if ctx.flags.reverse_sort {
                // reverse order
                my_files.sort_by(|a, b| b.filename.cmp(&a.filename));
            } else {
                my_files.sort_by(|a, b| a.filename.cmp(&b.filename));
            }
        }

        if ctx.flags.show_details {
            detailed_listing(&my_files, &ctx.flags);
        } else {
            simple_listing(&my_files, &ctx.flags);
        }
    }
}
