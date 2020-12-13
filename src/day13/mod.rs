use crate::utils;
use std::time::SystemTime;

fn part1(minimum_timestamp: u32, all_bus: &Vec<u32>) {
    let now = SystemTime::now();

    let mut result_id = all_bus[0];
    let mut result_timestamp = minimum_timestamp * 2;

    for bus in all_bus {
        let ratio = minimum_timestamp as f32 / *bus as f32;
        let bus_timestamp = ratio.ceil() as u32 * bus;
        if bus_timestamp < result_timestamp {
            result_id = *bus;
            result_timestamp = bus_timestamp;
        }
    }

    let result = result_id * (result_timestamp - minimum_timestamp);

    println!("Part 1: {}", result);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: String) {
    let now = SystemTime::now();

    println!("Part 2: {}", input.len());
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day13");

    let now = SystemTime::now();
    let input: String = utils::input::read("day13");
    let input_lines: Vec<&str> = input.split("\n").collect();

    let minimum_timestamp: u32 = input_lines[0].parse().unwrap();
    let all_bus_str: Vec<&str> = input_lines[1].split(",").collect();
    let mut all_bus: Vec<u32> = Vec::with_capacity(all_bus_str.len());

    for bus_str in all_bus_str {
        if bus_str == "x" {
            continue;
        }

        all_bus.push(bus_str.parse().unwrap());
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(minimum_timestamp, &all_bus);
    part2(input.clone());
}
