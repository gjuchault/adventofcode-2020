use crate::utils;
use std::collections::{HashMap, HashSet};
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

fn part1(input: &Vec<u64>) {
    let now = SystemTime::now();
    let preamble: usize = 25;
    let mut result = 0;

    for pos in preamble..input.len() {
        let start = pos - preamble;
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

    println!("Part 2: {}", 0);
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
