use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 6 solution.");

    println!("Parsing example input...");
    let input = parse_input("./inputs/day6/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);

    println!("Parsing input...");
    let input = parse_input("./inputs/day6/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);
}

fn solution(all_group_answers: &Vec<GroupAnswers>) -> i32 {
    let count_answered_question_per_group: Vec<i32> = all_group_answers
        .iter()
        .map(|group_answers| {
            (group_answers
                .answered_questions
                .iter()
                .filter(|answered| **answered)
                .count()) as i32
        })
        .collect();

    count_answered_question_per_group.iter().sum()
}

#[derive(Debug)]
pub struct GroupAnswers {
    pub answered_questions: Vec<bool>,
}

impl GroupAnswers {
    pub fn from_str(input_str: &String) -> io::Result<GroupAnswers> {
        let mut out = GroupAnswers {
            answered_questions: vec![false; 26],
        };

        input_str
            .chars()
            .for_each(|question_char| match question_char {
                ' ' => (),
                any_char => out.answered_questions[((any_char as i32) - 97) as usize] = true,
            });

        Ok(out)
    }
}

pub fn parse_input(filename: &str) -> io::Result<Vec<GroupAnswers>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut passports: Vec<GroupAnswers> = Vec::new();
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
                passports.push(GroupAnswers::from_str(&concat_lines(&lines))?);
                lines.clear();
            }
        }
    }

    passports.push(GroupAnswers::from_str(&concat_lines(&lines))?);

    Ok(passports)
}
