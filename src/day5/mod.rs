use crate::utils;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, PartialEq)]
enum BinaryChange {
    Upper,
    Lower,
}

impl fmt::Display for BinaryChange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinaryChange::Upper => write!(f, "U"),
            BinaryChange::Lower => write!(f, "L"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Ticket {
    row: u32,
    column: u32,
}

fn part1(tickets: Vec<Ticket>) {
    let now = SystemTime::now();

    let mut result = 0;
    for ticket in tickets {
        let score = ticket.row * 8 + ticket.column;

        if score > result {
            result = score
        }
    }

    println!("Part 1: {}", result);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(tickets: Vec<Ticket>) {
    let now = SystemTime::now();
    let max = 2u32.pow(10);
    let mut result = 0;

    let mut all_tickets_ids: Vec<u32> = Vec::with_capacity(max as usize);
    let mut given_tickets_ids: Vec<u32> = Vec::with_capacity(tickets.len());

    for i in 0..tickets.len() {
        given_tickets_ids.push(tickets[i].row * 8 + tickets[i].column);
    }

    for i in 0..max {
        let bin = format!("{:010b}", i);
        let mut ticket_str = String::with_capacity(10);

        for bin_index in 0..bin.len() {
            let bin_char = utils::string::char_at(bin.clone(), bin_index as u32, '_');

            if bin_index < 7 {
                ticket_str.push(if bin_char == '1' { 'B' } else { 'F' })
            } else {
                ticket_str.push(if bin_char == '1' { 'R' } else { 'L' })
            }
        }

        let ticket = parse_ticket(ticket_str);
        all_tickets_ids.push(ticket.row * 8 + ticket.column);
    }

    for i in 0..all_tickets_ids.len() {
        let score = all_tickets_ids[i];

        if !given_tickets_ids.contains(&score) && score > 10 && score < 930 {
            result = score;
        }
    }

    println!("Part 2: {}", result);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn find_binary(initial_max: u32, binary_changes: Vec<BinaryChange>) -> u32 {
    let mut min: u32 = 0;
    let mut max: u32 = initial_max;

    for binary_change in binary_changes {
        let (new_min, new_max) = take_next_binary_change(min, max, binary_change);

        min = new_min;
        max = new_max;
    }

    if min != max {
        panic!("Min != Max {} {}", min, max);
    }

    return min;
}

fn take_next_binary_change(min: u32, max: u32, binary_change: BinaryChange) -> (u32, u32) {
    let number_of_items = max + 1 - min;

    if binary_change == BinaryChange::Lower {
        return (min, min + number_of_items / 2 - 1);
    } else {
        return (min + number_of_items / 2, max);
    }
}

fn parse_ticket(input: String) -> Ticket {
    let mut row_binary_operations: Vec<BinaryChange> = Vec::with_capacity(7);
    let mut column_binary_operations: Vec<BinaryChange> = Vec::with_capacity(3);

    for character in 0..input.len() {
        let character_str = utils::string::char_at(input.clone(), character as u32, '_');
        match character_str {
            'B' => row_binary_operations.push(BinaryChange::Upper),
            'F' => row_binary_operations.push(BinaryChange::Lower),
            'R' => column_binary_operations.push(BinaryChange::Upper),
            'L' => column_binary_operations.push(BinaryChange::Lower),
            _ => panic!("Unexpected character {}", character_str),
        }
    }

    let row = find_binary(127, row_binary_operations);
    let column = find_binary(7, column_binary_operations);

    return Ticket { row, column };
}

pub fn run() {
    println!("Running day5");
    let now = SystemTime::now();
    let input: String = utils::input::read("day5");

    let mut tickets: Vec<Ticket> = Vec::with_capacity(input.matches("\n").count());
    for ticket_str in input.lines() {
        tickets.push(parse_ticket(String::from(ticket_str)));
    }
    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(tickets.clone());
    part2(tickets.clone());
}
