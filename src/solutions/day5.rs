use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 5 solution.");

    println!("Parsing example input...");
    let input = parse_input("./inputs/day4/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input, true);
    println!("Solution: {}", count);

    println!("Parsing input...");
    let input = parse_input("./inputs/day4/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input, false);
    println!("Solution: {}", count);
}

fn solution(passports: &Vec<BoardingPass>, detail: bool) -> i32 {
    if detail {
        println!("lol");
    }
    0
}

#[derive(Debug)]
struct BoardingPass {
    row: Option<i32>,
    column: Option<i32>,
    seat_id: Option<i32>,
}

impl BoardingPass {
    pub fn from_str(input_str: &String) -> io::Result<BoardingPass> {
        let mut out = BoardingPass {
            row: None,
            column: None,
            seat_id: None,
        };

        Ok(out)
    }
}

fn parse_input(filename: &str) -> io::Result<Vec<BoardingPass>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut passports: Vec<BoardingPass> = Vec::new();
    let mut lines: Vec<String> = Vec::new();

    fn concat_lines(lines: &Vec<String>) -> String {
        lines
            .iter()
            .fold("".to_string(), |acc, line| format!("{} {}", acc, line))
    }

    for line in (io::BufReader::new(&file)).lines() {
        if let Ok(value) = line {
            lines.push(value.trim().to_string());
            if (&lines[lines.len() - 1]).eq("") {
                passports.push(BoardingPass::from_str(&concat_lines(&lines))?);
                lines.clear();
            }
        }
    }

    passports.push(BoardingPass::from_str(&concat_lines(&lines))?);

    Ok(passports)
}
