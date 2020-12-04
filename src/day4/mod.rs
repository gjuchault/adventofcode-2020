use crate::utils;
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone)]
struct Passport {
    ecl: Option<String>,
    pid: Option<String>,
    eyr: Option<String>,
    hcl: Option<String>,
    byr: Option<String>,
    iyr: Option<String>,
    cid: Option<String>,
    hgt: Option<String>,
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(ecl: {}, pid: {}, eyr: {}, hcl: {}, byr: {}, iyr: {}, cid: {}, hgt: {})",
            self.ecl.as_ref().unwrap_or(&String::from("?")),
            self.pid.as_ref().unwrap_or(&String::from("?")),
            self.eyr.as_ref().unwrap_or(&String::from("?")),
            self.hcl.as_ref().unwrap_or(&String::from("?")),
            self.byr.as_ref().unwrap_or(&String::from("?")),
            self.iyr.as_ref().unwrap_or(&String::from("?")),
            self.cid.as_ref().unwrap_or(&String::from("?")),
            self.hgt.as_ref().unwrap_or(&String::from("?")),
        )
    }
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        if self.ecl.is_none()
            || self.pid.is_none()
            || self.eyr.is_none()
            || self.hcl.is_none()
            || self.byr.is_none()
            || self.iyr.is_none()
            || self.hgt.is_none()
        {
            return false;
        }

        return true;
    }

    pub fn are_fields_valid(&self) -> bool {
        if !self.clone().is_valid() {
            return false;
        }

        if !validate_numeric_value(self.byr.clone().unwrap(), "", 1920, 2002) {
            return false;
        }

        if !validate_numeric_value(self.iyr.clone().unwrap(), "", 2010, 2020) {
            return false;
        }

        if !validate_numeric_value(self.eyr.clone().unwrap(), "", 2020, 2030) {
            return false;
        }

        if !validate_numeric_value(self.hgt.clone().unwrap(), "cm", 150, 193) {
            if !validate_numeric_value(self.hgt.clone().unwrap(), "in", 59, 76) {
                return false;
            }
        }

        if !validate_color(self.hcl.clone().unwrap().to_lowercase()) {
            return false;
        }

        let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        if !valid_ecl.contains(&self.ecl.clone().unwrap().as_str()) {
            return false;
        }

        let pid_length = self.pid.clone().unwrap().len();
        if !utils::string::is_numeric(self.pid.clone().unwrap()) || pid_length != 9 {
            return false;
        }

        return true;
    }
}

fn validate_numeric_value(input: String, unit: &str, min: u32, max: u32) -> bool {
    if !input.ends_with(unit) {
        return false;
    }

    let without_unit = String::from(&input[0..input.len() - unit.len()]);
    let parse_result = without_unit.parse::<u32>();

    if parse_result.is_err() {
        return false;
    }

    let numerical_value = parse_result.unwrap();

    return numerical_value >= min && numerical_value <= max;
}

fn validate_color(input: String) -> bool {
    if !input.starts_with("#") {
        return false;
    }

    if input.len() != 7 {
        return false;
    }

    let hexa_characters: Vec<String> = (0..16).map(|v| format!("{:x}", v)).collect();

    for i in 1..input.len() {
        if !hexa_characters.contains(&String::from(&input[i..i + 1])) {
            return false;
        }
    }

    return true;
}

fn part1(passports: Vec<Passport>) {
    let now = SystemTime::now();
    let mut valid_passports = 0;
    for i in 0..passports.len() {
        if passports[i].is_valid() {
            valid_passports += 1;
        }
    }
    println!("Part 1: {}", valid_passports);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(passports: Vec<Passport>) {
    let now = SystemTime::now();
    let mut valid_passports = 0;
    for i in 0..passports.len() {
        if passports[i].are_fields_valid() {
            valid_passports += 1;
        }
    }
    println!("Part 2: {}", valid_passports);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn parse_passports(input: String) -> Vec<Passport> {
    let input_lines: Vec<&str> = input.split("\n\n").collect();

    let mut passports: Vec<Passport> = Vec::with_capacity(input_lines.len());

    for line in input_lines {
        let sanitized_line = line.clone().replace("\n", " ");
        let mappings: Vec<&str> = sanitized_line.split(" ").collect();

        let mut passport = Passport {
            ecl: None,
            pid: None,
            eyr: None,
            hcl: None,
            byr: None,
            iyr: None,
            cid: None,
            hgt: None,
        };

        for mapping in mappings {
            let items: Vec<&str> = mapping.split(":").collect();

            match items[0] {
                "ecl" => passport.ecl = Some(String::from(items[1])),
                "pid" => passport.pid = Some(items[1].parse().unwrap()),
                "eyr" => passport.eyr = Some(items[1].parse().unwrap()),
                "hcl" => passport.hcl = Some(String::from(items[1])),
                "byr" => passport.byr = Some(items[1].parse().unwrap()),
                "iyr" => passport.iyr = Some(items[1].parse().unwrap()),
                "cid" => passport.cid = Some(items[1].parse().unwrap()),
                "hgt" => passport.hgt = Some(String::from(items[1])),
                _ => panic!("Unexpected field {}", items[0]),
            }
        }

        passports.push(passport);
    }

    return passports;
}

pub fn run() {
    println!("Running day4");
    let now = SystemTime::now();
    let input: String = utils::input::read("day4");

    let passports = parse_passports(input);

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(passports.clone());
    part2(passports.clone());
}
