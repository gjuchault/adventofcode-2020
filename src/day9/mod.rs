use crate::utils;
use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TwoIndexes {
    left: usize,
    right: usize,
}

impl fmt::Display for TwoIndexes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}", self.left, self.right)
    }
}

const PREAMBLE: usize = 25;

fn part1(input: &Vec<u64>) {
    let now = SystemTime::now();
    let mut result = 0;

    for pos in PREAMBLE..input.len() {
        let start = pos - PREAMBLE;
        let stop = pos;
        let previous_values = Vec::from_iter(input[start..stop].iter().cloned());

        if !is_sum_in_list(input[pos], &previous_values) {
            result = input[pos];
            break;
        }
    }

    println!("Part 1: {}", result);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn is_sum_in_list(input: u64, list: &Vec<u64>) -> bool {
    let mut map: HashMap<usize, bool> = HashMap::with_capacity(list.len());

    for value in list {
        map.insert(*value as usize, true);
    }

    for value in list {
        if *value > input {
            continue;
        }

        let rest = input - *value;

        // do not allow x + x to validate y
        if rest == *value {
            continue;
        }

        if map.contains_key(&(rest as usize)) {
            return true;
        }
    }

    return false;
}

fn part2(input: &Vec<u64>) {
    let now = SystemTime::now();
    let mut target = 0;
    let mut target_pos: usize = 0;
    let mut result = 0;

    for pos in PREAMBLE..input.len() {
        let start = pos - PREAMBLE;
        let stop = pos;
        let previous_values = Vec::from_iter(input[start..stop].iter().cloned());

        if !is_sum_in_list(input[pos], &previous_values) {
            target = input[pos];
            target_pos = pos;
            break;
        }
    }

    for sequence_size in 2..target_pos {
        if result > 0 {
            break;
        }

        for sequence_start in 0..target_pos - sequence_size + 1 {
            let mut sum = 0;
            let mut min = input[sequence_start];
            let mut max = input[sequence_start];

            for i in sequence_start..sequence_start + sequence_size {
                if input[i] > max {
                    max = input[i];
                }

                if input[i] < min {
                    min = input[i];
                }

                sum += input[i];
            }

            if sum == target {
                result = min + max;
                break;
            }
        }
    }

    println!("Part 2: {}", result);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day9");

    let now = SystemTime::now();
    let input: String = utils::input::read("day9");
    let input_lines: Vec<&str> = input.split("\n").collect();
    let mut numbers: Vec<u64> = Vec::with_capacity(input_lines.len());

    for line in input_lines {
        numbers.push(line.parse().unwrap());
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(&numbers);
    part2(&numbers);
}
