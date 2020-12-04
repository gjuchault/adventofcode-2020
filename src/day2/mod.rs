use crate::utils;
use regex::Regex;
use std::time::SystemTime;

#[derive(Debug, Clone)]
struct PasswordLine {
    letter: char,
    min: u32,
    max: u32,
    challenge: String,
}

fn parse_password_line(input: &str, password_line_re: &regex::Regex) -> PasswordLine {
    let r: regex::Captures = password_line_re.captures(input).unwrap();

    assert_eq!(r.len(), 5);

    let min = r[1].parse::<u32>().unwrap();
    let max = r[2].parse::<u32>().unwrap();
    let letter = &r[3];
    let challenge = &r[4];

    return PasswordLine {
        letter: utils::string::char_at(String::from(letter), 0, '_'),
        min: min,
        max: max,
        challenge: String::from(challenge),
    };
}

fn is_password_line_valid(input: PasswordLine) -> bool {
    let mut occurences: u32 = 0;
    for character in input.challenge.chars() {
        if character == input.letter {
            occurences += 1;
        }
    }

    return occurences >= input.min && occurences <= input.max;
}

fn is_password_line_valid_2(input: PasswordLine) -> bool {
    let letter_at_min = utils::string::char_at(input.challenge.clone(), input.min - 1, '_');
    let letter_at_max = utils::string::char_at(input.challenge.clone(), input.max - 1, '_');

    let letter_min_match = letter_at_min == input.letter;
    let letter_max_match = letter_at_max == input.letter;

    return (letter_min_match || letter_max_match) && !(letter_min_match && letter_max_match);
}

fn part1(password_lines: Vec<PasswordLine>) {
    let now = SystemTime::now();

    let mut valid_passwords: u32 = 0;
    for password_line in password_lines {
        if is_password_line_valid(password_line) {
            valid_passwords += 1;
        }
    }

    println!("Part 1: {}", valid_passwords);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(password_lines: Vec<PasswordLine>) {
    let now = SystemTime::now();

    let mut valid_passwords: u32 = 0;
    for password_line in password_lines {
        if is_password_line_valid_2(password_line) {
            valid_passwords += 1;
        }
    }

    println!("Part 2: {}", valid_passwords);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

pub fn run() {
    println!("Running day2");
    let now = SystemTime::now();
    let input: String = utils::input::read("day2");
    let password_line_re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let password_lines_inputs: Vec<&str> = input.split("\n").collect();
    let mut password_lines: Vec<PasswordLine> = Vec::with_capacity(password_lines_inputs.len());
    for password_line_input in password_lines_inputs {
        password_lines.push(parse_password_line(password_line_input, &password_line_re));
    }
    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());
    part1(password_lines.clone());
    part2(password_lines.clone());
}
