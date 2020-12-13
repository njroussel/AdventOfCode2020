use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 2 solution.");

    println!("Parsing example input...");
    let input = parse_input("./inputs/day2/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let test_results: Vec<&Password> = input
        .iter()
        .filter(|&password| test_password(password))
        .collect();
    let count = test_results.len();

    println!("Solution: {}", count);

    let input = parse_input("./inputs/day2/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let test_results: Vec<&Password> = input
        .iter()
        .filter(|&password| test_password(password))
        .collect();
    let count = test_results.len();

    println!("Solution: {}", count);
}

fn test_password(password: &Password) -> bool {
    let mut counter: i32 = 0;

    for password_char in password.password.chars() {
        if password_char == password.policy_char {
            counter += 1;
            if counter > password.policy_max {
                return false;
            }
        }
    }

    if counter < password.policy_min {
        return false;
    }

    true
}

struct Password {
    password: String,
    policy_char: char,
    policy_min: i32,
    policy_max: i32,
}

fn parse_input(filename: &str) -> io::Result<Vec<Password>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut out: Vec<Password> = Vec::new();

    for line in (io::BufReader::new(&file)).lines() {
        if let Ok(value) = line {
            let parts: Vec<&str> = value.split(' ').collect();

            let policy_range: Vec<&str> = parts[0].split('-').collect();
            let policy_min: i32 = policy_range[0].parse().unwrap();
            let policy_max: i32 = policy_range[1].parse().unwrap();

            let policy_char = parts[1].split(':').collect::<Vec<&str>>()[0]
                .to_string()
                .chars()
                .collect::<Vec<char>>()[0];

            let password = parts[2];

            out.push(Password {
                password: password.to_string(),
                policy_char: policy_char,
                policy_min: policy_min,
                policy_max: policy_max,
            });
        }
    }

    Ok(out)
}
