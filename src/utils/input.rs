use std::env;
use std::fs;

pub fn read(day: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(format!("src/{}/input.txt", day));

    let contents = fs::read_to_string(path)
        .expect(String::from(format!("Couldn't read src/{}/input.txt input.txt", day)).as_str());

    return String::from(contents.trim());
}
