use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 1 solution.");

    println!("Parsing example input...");
    let input = parse_input("./inputs/day3/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);

    println!("Parsing input...");
    let input = parse_input("./inputs/day3/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);
}

fn solution(forest: &Forest) -> i32 {
    tree_counter(forest, (3, 1))
}

pub fn tree_counter(forest: &Forest, traversal_strategy: (i32, i32)) -> i32 {
    let (x_move, y_move) = traversal_strategy;
    let mut current_position: (i32, i32) = (0, 0);
    let mut reached_end = false;
    let mut counter: i32 = 0;

    while !reached_end {
        match forest.traverse(current_position, x_move, y_move) {
            Err(_) => reached_end = true,
            Ok(new_position) => {
                current_position = new_position;
                if forest.tree_at(current_position) {
                    counter += 1;
                }
            }
        }
    }

    counter
}

pub struct Forest {
    pub trees: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Forest {
    pub fn from_file(filename: &str, tree_char: char) -> io::Result<Forest> {
        let path = Path::new(filename);
        let file = File::open(&path)?;

        let mut forest: Vec<Vec<bool>> = Vec::new();

        for line in (io::BufReader::new(&file)).lines() {
            if let Ok(value) = line {
                let row: Vec<bool> = value.chars().map(|c| c == tree_char).collect();
                forest.push(row)
            }
        }

        let width = forest[0].len();
        let height = forest.len();

        Ok(Forest {
            trees: forest,
            width: width,
            height: height,
        })
    }

    pub fn tree_at(&self, position: (i32, i32)) -> bool {
        let (width, height) = position;
        self.trees[height as usize][width as usize]
    }

    pub fn traverse(
        &self,
        current_position: (i32, i32),
        x_move: i32,
        y_move: i32,
    ) -> Result<(i32, i32), &str> {
        let new_x = (current_position.0 + x_move) % (self.width as i32);
        let new_y = current_position.1 + y_move;

        if new_y >= (self.height as i32) {
            return Err("Trying to exit the Y axis of the forest!");
        }

        Ok((new_x, new_y))
    }
}

pub fn parse_input(filename: &str) -> io::Result<Forest> {
    Forest::from_file(filename, '#')
}
