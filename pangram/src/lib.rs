use std::ascii::AsciiExt;
use std::collections::HashSet;

pub fn is_pangram(string: &str) -> bool {
    let alphabet = string
        .to_string()
        .to_lowercase()
        .chars()
        .filter(|s| s.is_ascii() && s.is_alphabetic())
        .fold(HashSet::new(), |mut acc, ch| {
            acc.insert(ch);
            acc
        });
    alphabet.len() == 26
}
