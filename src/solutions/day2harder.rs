use crate::solutions::day2;

pub fn run() {
    println!("Day 2 solution (harder)");

    println!("Parsing example input...");
    let input = day2::parse_input("./inputs/day2/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let test_results: Vec<&day2::Password> = input
        .iter()
        .filter(|&password| test_password(password))
        .collect();
    let count = test_results.len();

    println!("Solution: {}", count);

    let input = day2::parse_input("./inputs/day2/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let test_results: Vec<&day2::Password> = input
        .iter()
        .filter(|&password| test_password(password))
        .collect();
    let count = test_results.len();

    println!("Solution: {}", count);
}

fn test_password(password: &day2::Password) -> bool {
    let password_chars: Vec<char> = password.password.chars().collect();

    let first_char = password_chars[(password.policy_min - 1) as usize];
    let second_char = password_chars[(password.policy_max - 1) as usize];

    let first_condition = first_char == password.policy_char;
    let second_condition = second_char == password.policy_char;

    first_condition ^ second_condition
}
