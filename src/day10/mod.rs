use crate::utils;
use std::time::SystemTime;

fn part1(input: &Vec<u32>) {
    let now = SystemTime::now();

    let mut diff1 = 1;
    let mut diff3 = 1;

    for i in 0..input.len() - 1 {
        if input[i + 1] - input[i] == 1 {
            diff1 += 1;
        }

        if input[i + 1] - input[i] == 3 {
            diff3 += 1;
        }
    }

    println!("Part 1: {}", diff1 * diff3);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: String) {
    let now = SystemTime::now();

    println!("Part 2: {}", 0);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day10");

    let now = SystemTime::now();
    let input: String = utils::input::read("day10");
    let input_lines: Vec<&str> = input.split("\n").collect();
    let mut adapters: Vec<u32> = Vec::with_capacity(input_lines.len());

    for line in input.split("\n") {
        adapters.push(line.parse().unwrap());
    }

    adapters.sort();

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());
    part1(&adapters);
    part2(input.clone());
}
