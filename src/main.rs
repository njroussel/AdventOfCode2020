mod argparser;
mod solutions;

fn main() {
    let (day, harder) = argparser::parse();

    match (day, harder) {
        (1, false) => solutions::day1::run(),
        (1, true) => solutions::day1harder::run(),
        (2, false) => solutions::day2::run(),
        (2, true) => solutions::day2harder::run(),
        (3, false) => solutions::day3::run(),
        (3, true) => solutions::day3harder::run(),
        (4, false) => solutions::day4::run(),
        (4, true) => solutions::day4harder::run(),
        (5, false) => solutions::day5::run(),
        (5, true) => solutions::day5harder::run(),
        (6, false) => solutions::day6::run(),
        (6, true) => solutions::day6harder::run(),
        _ => println!("No implementation for this day yet!"),
    }
}
