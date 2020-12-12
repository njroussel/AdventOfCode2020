use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    parse_input("./inputs/day1/example").unwrap();
    println!("Day 1 solution");
}

fn parse_input(filename: &str) -> io::Result<i32> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    for line in (io::BufReader::new(&file)).lines() {
        if let Ok(value) = line {
            println!("{}", value)
        }
    }

    Ok(1)
}
