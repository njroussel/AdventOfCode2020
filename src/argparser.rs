use clap::{App, Arg};

pub fn parse() -> (u32, bool) {
    let arg_matches = App::new("AdventOfCode2020")
        .version("1.0")
        .about("Solutions to AdventOfCode 2020")
        .arg(
            Arg::new("day")
                .about("The calendar day's solution to run")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("harder")
                .long("harder")
                .about("To solve the harder puzzle of the day"),
        )
        .get_matches();

    let day: u32 = arg_matches
        .value_of("day")
        .unwrap()
        .parse()
        .expect("Could not parse day value - expecting an integer!");

    let harder: bool = arg_matches.is_present("harder");

    if day < 1 || day > 24 {
        panic!("The calendar day should be between 1 and 24 (included)!");
    }

    println!("The selected day is: {}", day);
    if harder {
        println!("Solving the harder puzzle for day: {}", day);
    } else {
        println!("Solving the normal puzzle for day: {}", day);
    }

    (day, harder)
}
