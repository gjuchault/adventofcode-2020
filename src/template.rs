use crate::utils;
use std::time::SystemTime;

fn part1() {
    let now = SystemTime::now();
    let input: String = utils::input::read("dayX");

    println!("Part 1: {}", input);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2() {
    let now = SystemTime::now();
    let input: String = utils::input::read("dayX");

    println!("Part 2: {}", input);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running dayX");

    let now = SystemTime::now();
    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1();
    part2();
}
