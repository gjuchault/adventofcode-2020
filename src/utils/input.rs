use std::env;
use std::fs;

pub fn read(day: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(format!("src/{}/input.txt", day));

    let contents = fs::read_to_string(path).expect("Couldn't read day1 input.txt");

    return String::from(contents.trim());
}
