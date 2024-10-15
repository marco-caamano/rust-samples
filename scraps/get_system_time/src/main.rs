use chrono::{DateTime, Datelike, Local};
use std::time::SystemTime;

fn main() {
    println!("Hello, time!");

    let now = Local::now();

    println!("Current Time: {}", now);

    println!("Formated Time: {}", now.format("%Y/%m/%d %H:%M:%S"));

    let year = now.year();
    println!("Year: {}", year);

    println!("\n");

    println!("Using System Time");
    let stime = SystemTime::now();
    let ltime: DateTime<Local> = DateTime::from(stime);
    println!("Local Time: {}", ltime);
    println!("Formated Time: {}", ltime.format("%Y-%m-%d %H:%M:%S"));
    println!("Formated Time: {}", ltime.format("%Y-%b-%d %H:%M:%S"));
}
