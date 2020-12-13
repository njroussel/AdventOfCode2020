use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 1 solution.");

    println!("Parsing example input...");
    let input = parse_input("./inputs/day1/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let result = naive_solution(&input, 2020);
    match result {
        Ok(val) => println!("Solution: {}", val),
        Err(e) => println!("{}", e),
    }

    println!("Parsing input...");
    let input = parse_input("./inputs/day1/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let result = naive_solution(&input, 2020);
    match result {
        Ok(val) => println!("Solution: {}", val),
        Err(e) => println!("{}", e),
    }
}

fn naive_solution(vec: &Vec<i32>, goal: i32) -> Result<i32, &str> {
    let len = vec.len();
    for i in 0..len {
        let i_val = vec[i];
        for j in i + 1..len {
            let j_val = vec[j];
            if i_val + j_val == goal {
                println!("Found factors: {} and {}", i_val, j_val);
                return Ok(i_val * j_val);
            }
        }
    }

    Err("No solution found")
}

pub fn parse_input(filename: &str) -> io::Result<Vec<i32>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut out: Vec<i32> = Vec::new();

    for line in (io::BufReader::new(&file)).lines() {
        if let Ok(value) = line {
            out.push(value.parse().unwrap());
        }
    }

    Ok(out)
}
