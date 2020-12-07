use crate::utils;
use std::collections::HashSet;
use std::time::SystemTime;

fn part1(groups_answers: Vec<HashSet<char>>) {
    let now = SystemTime::now();

    let mut score = 0;
    for group_answers in groups_answers {
        score += group_answers.len();
    }

    println!("Part 1: {}", score);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: String) {
    let now = SystemTime::now();

    // println!("Part 2: {}", input);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day6");

    let now = SystemTime::now();
    let input: String = utils::input::read("day6");
    let groups: Vec<&str> = input.split("\n\n").collect();
    let mut groups_answers: Vec<HashSet<char>> = Vec::with_capacity(groups.len());

    for group in groups {
        let mut group_hash: HashSet<char> = HashSet::new();

        for character in group.chars() {
            if character == '\n' {
                continue;
            }

            group_hash.insert(character);
        }

        groups_answers.push(group_hash);
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(groups_answers.clone());
    part2(input.clone());
}
