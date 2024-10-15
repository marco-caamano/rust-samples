use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use std::{
    env, fs,
    os::unix::fs::{FileTypeExt, MetadataExt},
};

fn main() {
    println!("Hello, world!");

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
    let filetype = metadata.file_type();

    println!("File size: {} bytes\n", metadata.len());

    println!("------------------");
    println!("Using metdata");
    println!("------------------");
    println!("Is Dir      {:?}", metadata.is_dir());
    println!("Is File     {:?}", metadata.is_file());
    println!("Is Symlink  {:?}", metadata.is_symlink());

    println!("");

    println!("------------------");
    println!("Using metadata.filetype");
    println!("------------------");
    println!("Is Dir      {:?}", filetype.is_dir());
    println!("Is File     {:?}", filetype.is_file());
    println!("Is Symlink  {:?}", filetype.is_symlink());
    println!("Is Char     {:?}", filetype.is_char_device());
    println!("Is Block    {:?}", filetype.is_block_device());

    println!("");

    let smeta = match fs::symlink_metadata(&target) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("ERROR failed to read symlink metadata: {}", e);
            std::process::exit(1);
        }
    };

    println!("------------------");
    println!("Using symlink_metadata");
    println!("------------------");
    println!("Is Dir      {:?}", smeta.is_dir());
    println!("Is File     {:?}", smeta.is_file());
    println!("Is Symlink  {:?}", smeta.is_symlink());

    let mode = smeta.mode();
    let perms = parse_mode(mode);

    println!("\nmode: [{mode}] | perms[{perms}]\n");
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
        perm.push('x');
    } else {
        perm.push('-');
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
        perm.push('x');
    } else {
        perm.push('-');
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
        perm.push('x');
    } else {
        perm.push('-');
    }

    perm
}
