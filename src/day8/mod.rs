use crate::utils;
use std::collections::HashSet;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Copy, Clone)]
struct Command {
    kind: CommandKind,
    argument: i32,
}

#[derive(Debug, Copy, Clone)]
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

fn part1(input: &Vec<Command>) {
    let now = SystemTime::now();

    let mut indexes_encountered: HashSet<usize> = HashSet::new();
    let mut accumulator: i32 = 0;
    let mut command_index: usize = 0;
    loop {
        if indexes_encountered.contains(&command_index) {
            break;
        }

        indexes_encountered.insert(command_index);

        let command = input[command_index];

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

    println!("Part 1: {}", accumulator);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(input: String) {
    let now = SystemTime::now();

    println!("Part 2: {}", 0);
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
    part2(input.clone());
}
