use crate::utils;
use std::collections::HashSet;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone)]
struct Command {
    kind: CommandKind,
    argument: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CommandKind {
    Nop,
    Acc,
    Jmp,
}

impl fmt::Display for CommandKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandKind::Nop => write!(f, "nop"),
            CommandKind::Acc => write!(f, "acc"),
            CommandKind::Jmp => write!(f, "jmp"),
        }
    }
}

fn run_commands(input: &Vec<Command>, command_index_to_swap: Option<usize>) -> (bool, i32) {
    let mut indexes_encountered: HashSet<usize> = HashSet::with_capacity(input.len());
    let mut accumulator: i32 = 0;
    let mut command_index: usize = 0;
    loop {
        if indexes_encountered.contains(&command_index) {
            return (true, accumulator);
        }

        if command_index >= input.len() {
            return (false, accumulator);
        }

        indexes_encountered.insert(command_index);

        let mut command = input[command_index];

        if command_index_to_swap.is_some()
            && command_index_to_swap.unwrap() == command_index
            && command.kind != CommandKind::Acc
        {
            if command.kind == CommandKind::Jmp {
                command.kind = CommandKind::Nop;
            } else {
                command.kind = CommandKind::Jmp;
            }
        }

        match command.kind {
            CommandKind::Nop => {
                command_index += 1;
            }
            CommandKind::Acc => {
                accumulator += command.argument;
                command_index += 1;
            }
            CommandKind::Jmp => {
                command_index = (command_index as i32 + command.argument) as usize;
            }
        }
    }
}

fn part1(input: &Vec<Command>) {
    let now = SystemTime::now();

    let (_, accumulator) = run_commands(input, None);

    println!("Part 1: {}", accumulator);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: &Vec<Command>) {
    let now = SystemTime::now();
    let mut result = 0;
    let mut current_command_swapped =
        utils::vec::find_index(input, |command, _| command.kind != CommandKind::Acc).unwrap();

    loop {
        let (is_infinite_loop, accumulator) = run_commands(input, Some(current_command_swapped));

        if !is_infinite_loop {
            result = accumulator;
            break;
        }

        let next_command_to_swap = utils::vec::find_index(input, |command, i| {
            command.kind != CommandKind::Acc && i > current_command_swapped
        });

        if next_command_to_swap.is_none() {
            panic!("Couldn't find another swapping to do");
        }

        current_command_swapped = next_command_to_swap.unwrap();
    }

    println!("Part 2: {}", result);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day8");

    let now = SystemTime::now();
    let input: String = utils::input::read("day8");
    let input_lines: Vec<&str> = input.split("\n").collect();

    let mut commands: Vec<Command> = Vec::with_capacity(input_lines.len());

    for line in input_lines {
        let kind: CommandKind = match &line[0..3] {
            "nop" => CommandKind::Nop,
            "acc" => CommandKind::Acc,
            "jmp" => CommandKind::Jmp,
            _ => panic!("Unknown command {}", &line[0..3]),
        };

        let argument: i32 = (&line[4..]).parse().unwrap();

        commands.push(Command { kind, argument })
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(&commands);
    part2(&commands);
}
