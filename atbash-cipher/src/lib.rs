use std::ascii::AsciiExt;

fn cipher(ch: char) -> char {
    match ch.is_digit(10) {
        true => ch,
        false => ('z' as u8 - ch as u8 + 'a' as u8) as char,
    }
}


pub fn encode(string: & str) -> String {
    string
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii() && ch.is_alphanumeric())
        .map(|x| cipher(x))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|slice| slice.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(string: &str) -> String {
    string
        .split_whitespace()
        .collect::<String>()
        .chars()
        .map(|ch| cipher(ch))
        .collect::<String>()
}
