use crate::solutions::day5;
use std::cmp;

pub fn run() {
    println!("Day 5 solution (harder).");

    println!("Parsing input...");
    let input = day5::parse_input("./inputs/day5/input").unwrap();
    println!("Parsed.");

    println!("Solving...");
    let count = solution(&input, true);
    println!("Solution: {}", count);
}

fn solution(boardingpasses: &Vec<day5::BoardingPass>, detail: bool) -> i32 {
    let mut occupied_seats: Vec<bool> = vec![false; 8 * 127 + 7];

    boardingpasses
        .iter()
        .for_each(|boardingpass| occupied_seats[boardingpass.seat_id as usize] = true);

    let max_seat_id = boardingpasses
        .iter()
        .fold(0, |max, boardinpass| cmp::max(max, boardinpass.seat_id));

    let min_seat_id = boardingpasses.iter().fold(8 * 127 + 7, |max, boardinpass| {
        cmp::min(max, boardinpass.seat_id)
    });

    let unoccupied_seats: Vec<(usize, &bool)> = occupied_seats
        .iter()
        .enumerate()
        .filter(|(i, _)| i >= &(min_seat_id as usize) && i <= &(max_seat_id as usize))
        .filter(|(_, occupied)| !**occupied)
        .collect();

    if detail {
        unoccupied_seats
            .iter()
            .for_each(|(i, occupied)| println!("{} {}", i, occupied));
    }

    unoccupied_seats[0].0 as i32
}
