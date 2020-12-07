use crate::utils;
use std::collections::HashSet;
use std::time::SystemTime;

fn part1(groups: Vec<Vec<HashSet<char>>>) {
    let now = SystemTime::now();

    let mut score = 0;

    for group in groups {
        let mut group_hash: HashSet<char> = HashSet::new();

        for people in group {
            for answer in people {
                group_hash.insert(answer);
            }
        }

        score += group_hash.len();
    }

    println!("Part 1: {}", score);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(groups: Vec<Vec<HashSet<char>>>) {
    let now = SystemTime::now();

    let mut score = 0;

    for group in groups {
        for i in 97..123 as u8 {
            let mut all_people_have_answered = true;
            for people in &group {
                if !people.contains(&(i as char)) {
                    all_people_have_answered = false;
                }
            }

            if all_people_have_answered {
                score += 1;
            }
        }
    }

    println!("Part 2: {}", score);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day6");

    let now = SystemTime::now();
    let input: String = utils::input::read("day6");
    let groups_str: Vec<&str> = input.split("\n\n").collect();
    let mut groups: Vec<Vec<HashSet<char>>> = Vec::with_capacity(groups_str.len());

    for group_str in groups_str {
        let peoples_str: Vec<&str> = group_str.split("\n").collect();
        let mut group: Vec<HashSet<char>> = Vec::with_capacity(peoples_str.len());

        for people_str in peoples_str {
            let mut people: HashSet<char> = HashSet::new();

            for character in people_str.chars() {
                people.insert(character);
            }

            group.push(people);
        }

        groups.push(group);
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(groups.clone());
    part2(groups.clone());
}
