use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 5 solution.");

    println!("Parsing example input...");
    let input = parse_input("./inputs/day5/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input, true);
    println!("Solution: {}", count);

    println!("Parsing input...");
    let input = parse_input("./inputs/day5/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input, false);
    println!("Solution: {}", count);
}

fn solution(boardingpasses: &Vec<BoardingPass>, detail: bool) -> i32 {
    if detail {
        for boardingpass in boardingpasses {
            println!("{:?}", boardingpass)
        }
    }

    boardingpasses
        .iter()
        .fold(0, |max, boardinpass| cmp::max(max, boardinpass.seat_id))
}

#[derive(Debug)]
pub struct BoardingPass {
    pub row: i32,
    pub column: i32,
    pub seat_id: i32,
}

impl BoardingPass {
    pub fn from_str(input_str: &String) -> io::Result<BoardingPass> {
        let mut out = BoardingPass {
            row: 0,
            column: 0,
            seat_id: 0,
        };

        let row_chars: Vec<char> = input_str.get(..7).unwrap().chars().collect();
        let column_chars: Vec<char> = input_str.get(7..).unwrap().chars().collect();

        let row_partioning_idx: Vec<bool> = row_chars.iter().map(|x| *x == 'F').collect();
        let row_idx = bsp(&row_partioning_idx, 0, 0, 127);

        let column_partioning_idx: Vec<bool> = column_chars.iter().map(|x| *x == 'L').collect();
        let column_idx = bsp(&column_partioning_idx, 0, 0, 7);

        out.row = row_idx;
        out.column = column_idx;
        out.seat_id = out.row * 8 + out.column;

        Ok(out)
    }
}

fn bsp(partioning_idx: &Vec<bool>, partition_idx: usize, start_idx: i32, end_idx: i32) -> i32 {
    let len = end_idx - start_idx + 1;

    if len == 2 {
        match partioning_idx[partition_idx] {
            true => return start_idx,
            false => return end_idx,
        }
    }

    let half_len = len / 2;
    match partioning_idx[partition_idx] {
        true => {
            return bsp(
                partioning_idx,
                partition_idx + 1,
                start_idx,
                end_idx - half_len,
            )
        }
        false => {
            return bsp(
                partioning_idx,
                partition_idx + 1,
                start_idx + half_len,
                end_idx,
            )
        }
    }
}

pub fn parse_input(filename: &str) -> io::Result<Vec<BoardingPass>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut boardingpasses: Vec<BoardingPass> = Vec::new();

    for line in (io::BufReader::new(&file)).lines() {
        if let Ok(value) = line {
            boardingpasses.push(BoardingPass::from_str(&value)?);
        }
    }

    Ok(boardingpasses)
}
