mod argparser;
mod solutions;

fn main() {
    let (day, harder) = argparser::parse();

    match (day, harder) {
        (1, false) => solutions::day1::run(),
        (1, true) => solutions::day1harder::run(),
        (2, false) => solutions::day2::run(),
        _ => println!("No implementation for this day yet!"),
    }
}
