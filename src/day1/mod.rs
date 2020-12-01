use crate::utils;

fn part1() {
    let input: String = utils::read_input::read("day1");

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
                return;
            }
        }
    }
}

fn part2() {
    let input: String = utils::read_input::read("day1");
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
                    return;
                }
            }
        }
    }
}

pub fn run() {
    println!("Running day1");

    part1();
    part2();
}
