mod argparser;

fn main() {
    let (day, harder) = argparser::parse();

    match day {
        1 => println!("LOL"),
        _ => println!("No implementation for this day yet!"),
    }
}
