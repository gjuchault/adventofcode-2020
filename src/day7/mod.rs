use crate::utils;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone)]
struct BagContent {
    color: String,
    count: u32,
}
impl PartialEq for BagContent {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}
impl Eq for BagContent {}

impl fmt::Display for BagContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.count, self.color.clone())
    }
}

fn part1(input: HashMap<String, Vec<BagContent>>) {
    let now = SystemTime::now();

    let mut queue: HashSet<String> = count_reverse(String::from("shiny gold"), &input);

    loop {
        let mut did_push_to_queue = false;

        for item in queue.clone() {
            let to_push: HashSet<String> = count_reverse(item.clone(), &input);

            if to_push.len() > 0 {
                for new_item in to_push {
                    did_push_to_queue = did_push_to_queue || queue.insert(new_item);
                }
            }
        }

        if !did_push_to_queue {
            break;
        }
    }

    println!("Part 1: {}", queue.len());
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn count_reverse(lf_color: String, full_map: &HashMap<String, Vec<BagContent>>) -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::new();
    let compare_to = BagContent {
        color: lf_color.clone(),
        count: 0,
    };

    // not optimized because of only parent -> child edge
    for (color, dependencies) in full_map {
        if dependencies.contains(&compare_to) {
            result.insert((*color).clone());
        }
    }

    return result;
}

fn part2(input: HashMap<String, Vec<BagContent>>) {
    let now = SystemTime::now();

    let score = count(String::from("shiny gold"), &input);

    println!("Part 2: {}", score);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn count(color: String, input: &HashMap<String, Vec<BagContent>>) -> u32 {
    let mut score = 0;

    let dependencies = input.get(&color).unwrap();

    if dependencies.len() == 0 {
        return 0;
    }

    for dependency in dependencies {
        let dep_score = count(dependency.color.clone(), input);
        score += dependency.count * dep_score;
        score += dependency.count;
    }

    return score;
}

pub fn run() {
    println!("Running day7");

    let now = SystemTime::now();
    let input: String = utils::input::read("day7");

    let mut bags_depencies: HashMap<String, Vec<BagContent>> = HashMap::new();

    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let left_color = String::from(parts[0]);
        let bags_inside: Vec<&str> = parts[1].split(", ").collect();

        let mut all_bag_contents: Vec<BagContent> = Vec::new();

        if bags_inside[0] == "no other bags." {
            bags_depencies.insert(left_color, all_bag_contents);
            continue;
        }

        for bag_inside in bags_inside {
            let cleaned_bag_inside = String::from(bag_inside)
                .replace(" bags.", "")
                .replace(" bag.", "")
                .replace(" bags", "")
                .replace(" bag", "");

            let count_char = utils::string::char_at(cleaned_bag_inside.clone(), 0, '0');
            let count: u32 = String::from(count_char).as_str().parse().unwrap();

            let bag_content = BagContent {
                count,
                color: String::from(&cleaned_bag_inside[2..]),
            };

            all_bag_contents.push(bag_content);
        }

        bags_depencies.insert(left_color, all_bag_contents);
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    // part1(bags_depencies.clone());
    part2(bags_depencies.clone());
}
