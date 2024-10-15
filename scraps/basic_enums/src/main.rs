use serde::Deserialize;
use std::env;
use std::fs::File;
use std::process::exit;

enum Command {
    Play,
    Pause,
    Stop,
    SkipFwd(u32), // milliseconds to skip forward
    SkipBwd(u32), // milliseconds to skip backward
    Jump(u32),    // jump to ms offset from start
    GetCurrentPost,
    Exit,
}

#[derive(Deserialize, Debug)]
struct SongSpec {
    name: String,
    duration: u32,
}

fn read_song_spec_from_file(path: &String) -> Result<SongSpec, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open file[{path}]: {}", e))?;
    let song: Result<SongSpec, String> =
        serde_json::from_reader(file).map_err(|e| format!("Failed to parse file[{path}]: {}", e));
    song
}

fn main() {
    println!("Hello, Basic Enums!");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("ERROR: Need to pass a song spec json file as a command line parameter");
        exit(1);
    }
    let filepath = &args[1];

    let song = read_song_spec_from_file(filepath);
    match song {
        Ok(song) => println!(
            "Read song spec OK\n\nName: {}\nDuration: {}\n\n",
            song.name, song.duration
        ),
        Err(e) => {
            println!("ERROR: {e} | From[{}:{}]", file!(), line!());
            exit(1);
        }
    }
}
