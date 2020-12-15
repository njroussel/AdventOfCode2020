use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 4 solution.");

    println!("Parsing example input...");
    let input = parse_input("./inputs/day4/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);

    println!("Parsing input...");
    let input = parse_input("./inputs/day4/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);
}

fn solution(passports: &Vec<Passport>) -> i32 {
    let mut counter: i32 = 0;

    for passport in passports {
        let is_valid = passport.byr.is_some()
            && passport.iyr.is_some()
            && passport.eyr.is_some()
            && passport.hgt.is_some()
            && passport.hcl.is_some()
            && passport.ecl.is_some()
            && passport.pid.is_some();

        if is_valid {
            counter += 1;
        }
    }

    counter
}

#[derive(Debug)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<i32>,
}

impl Passport {
    pub fn from_str(input_str: &String) -> io::Result<Passport> {
        let mut out = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        let kv_pairs = input_str.trim().split(" ");

        for kv_pair in kv_pairs {
            let kv: Vec<&str> = kv_pair.split(":").collect();
            let key = kv[0];
            let value = kv[1];

            match key {
                "byr" => out.byr = Some(value.parse().unwrap()),
                "iyr" => out.iyr = Some(value.parse().unwrap()),
                "eyr" => out.eyr = Some(value.parse().unwrap()),
                "hgt" => out.hgt = Some(value.parse().unwrap()),
                "hcl" => out.hcl = Some(value.parse().unwrap()),
                "ecl" => out.ecl = Some(value.parse().unwrap()),
                "pid" => out.pid = Some(value.parse().unwrap()),
                "cid" => out.cid = Some(value.parse().unwrap()),
                _ => (),
            }
        }

        Ok(out)
    }
}

fn parse_input(filename: &str) -> io::Result<Vec<Passport>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    let mut passports: Vec<Passport> = Vec::new();
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
                passports.push(Passport::from_str(&concat_lines(&lines))?);
                lines.clear();
            }
        }
    }

    passports.push(Passport::from_str(&concat_lines(&lines))?);

    Ok(passports)
}
