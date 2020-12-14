use crate::solutions::day3;

pub fn run() {
    println!("Day 3 solution (harder).");

    println!("Parsing example input...");
    let input = day3::parse_input("./inputs/day3/example").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);

    println!("Parsing input...");
    let input = day3::parse_input("./inputs/day3/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input);
    println!("Solution: {}", count);
}

fn solution(forest: &day3::Forest) -> i64 {
    let traversal_stregies = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product_counters: i64 = traversal_stregies
        .iter()
        .map(|strategy| day3::tree_counter(forest, *strategy))
        .fold(1, |product, counter| product * (counter as i64));

    product_counters
}
