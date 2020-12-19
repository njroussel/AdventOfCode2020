use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    println!("Day 4 solution (harder).");

    println!("Parsing example input (4 valid)...");
    let input = parse_input("./inputs/day4/examplehardervalid").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);

    println!("Parsing example input (4 invalid)...");
    let input = parse_input("./inputs/day4/exampleharderinvalid").unwrap();
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
        let has_all_fields = passport.byr.is_some()
            && passport.iyr.is_some()
            && passport.eyr.is_some()
            && passport.hgt.is_some()
            && passport.hcl.is_some()
            && passport.ecl.is_some()
            && passport.pid.is_some();

        if !has_all_fields {
            continue;
        }

        let byr = passport.byr.unwrap();
        if byr < 1920 || byr > 2002 {
            continue;
        }

        let iyr = passport.iyr.unwrap();
        if iyr < 2010 || iyr > 2020 {
            continue;
        }

        let eyr = passport.eyr.unwrap();
        if eyr < 2020 || eyr > 2030 {
            continue;
        }

        let hgtstr = passport.hgt.as_ref().unwrap();
        let hgtstr_len = hgtstr.len();
        if hgtstr_len < 4 {
            continue;
        }
        let unit = hgtstr.get(hgtstr_len - 2..hgtstr_len).unwrap();
        let value: i32 = hgtstr.get(..hgtstr_len - 2).unwrap().parse().unwrap();
        let invalid_hgt = match unit {
            "cm" => value < 150 || value > 193,
            "in" => value < 59 || value > 76,
            _ => true,
        };
        if invalid_hgt {
            continue;
        }

        let hcl = passport.hcl.as_ref().unwrap();
        let hcl_chars: Vec<char> = hcl.chars().collect();
        if hcl_chars[0] != '#' || hcl_chars.len() != 7 {
            continue;
        }
        let mut invalid_hcl = false;
        let hcl_numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let hcl_alphabet = ['a', 'b', 'c', 'd', 'e', 'f'];
        for i in 1..7 {
            let hcl_char = hcl_chars[i];
            if !hcl_numbers.contains(&hcl_char) && !hcl_alphabet.contains(&hcl_char) {
                invalid_hcl = true;
                break;
            }
        }
        if invalid_hcl {
            continue;
        }

        let ecl = passport.ecl.as_ref().unwrap();
        let ecl_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !ecl_values.contains(&ecl.as_str()) {
            continue;
        }

        let pid = passport.pid.as_ref().unwrap();
        let pid_chars: Vec<char> = pid.chars().collect();
        if pid_chars.len() != 9 {
            continue;
        }
        let mut invalid_pid = false;
        let pid_numbers = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for pid_char in pid_chars {
            if !pid_numbers.contains(&pid_char) {
                invalid_pid = true;
                break;
            }
        }
        if invalid_pid {
            continue;
        }

        counter += 1;
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
