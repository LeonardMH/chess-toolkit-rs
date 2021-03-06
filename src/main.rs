#[macro_use]
extern crate more_asserts;

use clap::{Arg, App};
use serde_json;

pub mod board;
pub mod piece;
pub mod serialization;
pub mod timer;

fn main() {
    let matches = App::new("Chess Toolkit (Rust)")
        .version("0.1")
        .author("Michael Leonard <maybeillrememberit@gmail.com")
        .about("An experimental chess toolkit written in Rust")
        .arg(Arg::with_name("display")
            .help("Display the given board-file")
            .takes_value(true))
        .get_matches();

    let display = matches.value_of("display").unwrap();
    println!("{}", display);
}
