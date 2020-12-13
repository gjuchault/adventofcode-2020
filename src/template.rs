use crate::utils;
use std::time::SystemTime;

fn part1(input: String) {
    let now = SystemTime::now();

    println!("Part 1: {}", input.len());
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: String) {
    let now = SystemTime::now();

    println!("Part 2: {}", input.len());
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running dayX");

    let now = SystemTime::now();
    let input: String = utils::input::read("dayX");
    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(input.clone());
    part2(input.clone());
}
