use crate::solutions::day1;

pub fn run() {
    println!("Day 1 solution (harder)");

    println!("Parsing example input...");
    let input = day1::parse_input("./inputs/day1/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let result = naive_solution(&input, 2020);
    match result {
        Ok(val) => println!("Solution: {}", val),
        Err(e) => println!("{}", e),
    }

    println!("Parsing input...");
    let input = day1::parse_input("./inputs/day1/input").unwrap();
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
            for k in j + 1..len {
                let k_val = vec[k];
                if i_val + j_val + k_val == goal {
                    println!("Found factors: {}, {}, {}", i_val, j_val, k_val);
                    return Ok(i_val * j_val * k_val);
                }
            }
        }
    }

    Err("No solution found")
}
