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
    pub fn is_valid(&self, enforce_cid: bool) -> bool {
        if enforce_cid && self.cid.is_none() {
            return false;
        }

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
}

fn part1(passports: Vec<Passport>) {
    let now = SystemTime::now();
    let mut valid_passports = 0;
    for i in 0..passports.len() {
        if passports[i].is_valid(false) {
            valid_passports += 1;
        }
    }
    println!("Part 1: {}", valid_passports);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(passports: Vec<Passport>) {
    let now = SystemTime::now();
    let result = 0;
    println!("Part 2: {}", result);
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
