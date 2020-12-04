pub fn char_at(input: String, index: u32, default: char) -> char {
    return input.clone().chars().nth(index as usize).unwrap_or(default);
}

pub fn count_char(input: String, sub: char) -> u32 {
    let mut occurences: u32 = 0;

    for c in input.chars() {
        if c == sub {
            occurences += 1
        }
    }

    return occurences;
}

pub fn is_numeric(input: String) -> bool {
    let valid_numeric_characters = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for c in input.chars() {
        if !valid_numeric_characters.contains(&c) {
            return false;
        }
    }

    return true;
}
