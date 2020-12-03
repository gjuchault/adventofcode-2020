pub fn char_at(input: String, index: u32, default: char) -> char {
    return input.clone().chars().nth(index as usize).unwrap_or(default);
}
