use crate::utils;
use std::time::SystemTime;

fn part1() {
    let now = SystemTime::now();
    let input: String = utils::input::read("day1");

    let numbers: Vec<&str> = input.split("\n").collect();

    for i in 0..numbers.len() {
        let first: u32 = numbers[i].parse().unwrap();

        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let iteratee: u32 = numbers[j].parse().unwrap();

            if first + iteratee == 2020 {
                println!("Part 1: {}", first * iteratee);
                println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
                return;
            }
        }
    }
}

fn part2() {
    let now = SystemTime::now();
    let input: String = utils::input::read("day1");
    let numbers: Vec<&str> = input.split("\n").collect();

    for i in 0..numbers.len() {
        let first: u32 = numbers[i].parse().unwrap();

        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let second: u32 = numbers[j].parse().unwrap();

            for u in 0..numbers.len() {
                if i == u || j == u {
                    continue;
                }

                let item: u32 = numbers[u].parse().unwrap();
                if first + second + item == 2020 {
                    println!("Part 2: {}", first * second * item);
                    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
                    return;
                }
            }
        }
    }
}

pub fn run() {
    println!("Running day1");

    let now = SystemTime::now();
    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1();
    part2();
}
