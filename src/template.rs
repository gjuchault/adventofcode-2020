use crate::utils;
use std::time::SystemTime;

fn part1(input: String) {
    let now = SystemTime::now();

    println!("Part 1: {}", input);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: String) {
    let now = SystemTime::now();

    println!("Part 2: {}", input);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running 6");

    let now = SystemTime::now();
    let input: String = utils::input::read("6");
    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(input.clone());
    part2(input.clone());
}
