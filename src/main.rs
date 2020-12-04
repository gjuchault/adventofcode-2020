use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

mod utils {
    pub mod grid;
    pub mod input;
    pub mod string;
}

fn main() {
    println!("Advent of code 2020!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("You must give the day you want to run. Eg: cargo run day1");
        std::process::exit(1);
    }

    let day: String = args[1].clone();

    match day.as_str() {
        "day1" => day1::run(),
        "day2" => day2::run(),
        "day3" => day3::run(),
        "day4" => day4::run(),
        _ => println!("Day not found"),
    }

    std::process::exit(0);
}
