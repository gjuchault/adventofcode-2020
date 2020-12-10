use crate::utils;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::time::SystemTime;

fn part1(input: &Vec<u32>) {
    let now = SystemTime::now();

    let mut diff1 = 0;
    let mut diff3 = 0;

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

fn part2(adapters: &Vec<u32>) {
    let now = SystemTime::now();

    let set: HashSet<u32> = HashSet::from_iter(adapters.iter().cloned());

    // The possibilities to get to 10 is the sum of the possibilities to get
    // to 7, 8 and 9. Hence we keep in memory the last 3 values (there's no need
    // for more).
    // As the first elements (0, 1) have always one way to be accessed, the
    // first sum of the memory should be 1.
    let mut memory = [0, 0, 1];

    for i in 1..adapters[adapters.len() - 1] - 2 {
        let mut next: u64 = 0;

        // As we're incrementally growing, we need to check that the set
        // contains the number we're checking. For example:
        // 5 6 7 10
        // As 10 is the sum of possibilities of 7 (2 here) 8 (0) and 9 (0)
        // we need to push as many 0 as there are missing numbers.
        if set.contains(&(i as u32)) {
            next = memory[0] + memory[1] + memory[2];
        }

        memory[0] = memory[1];
        memory[1] = memory[2];
        memory[2] = next;
    }

    println!("Part 2: {}", memory[2]);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day10");

    let now = SystemTime::now();
    let input: String = utils::input::read("day10");
    let input_lines: Vec<&str> = input.split("\n").collect();
    let mut adapters: Vec<u32> = Vec::with_capacity(input_lines.len() + 2);

    adapters.push(0);
    for line in input.split("\n") {
        adapters.push(line.parse().unwrap());
    }

    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());
    part1(&adapters);
    part2(&adapters);
}
