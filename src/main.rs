// #[macro_use]
// extern crate nickel;

use chrono::*;
use nickel::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn log_time() -> String {
    let local: DateTime<Local> = Local::now();
    let date = local.format("%a, %d %b %Y %I:%M:%S %p\n").to_string();
    date
}

fn log_time_into_file(filename: String) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    f.write_all(log_time().as_bytes())?;
    Ok(())
}

fn do_log_time() -> String {
    match log_time_into_file("log.txt".to_string()) {
        Ok(..) => println!("Timestamp logged: {:?}", log_time()),
        Err(..) => println!("Oh no no no no no"),
    }
    "Hello World! The visiting timestamp has been logged.".to_string()
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time()
        }
    });

    server.listen("0.0.0.0:6767").unwrap();
}
