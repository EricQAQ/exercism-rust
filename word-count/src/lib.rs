use std::collections::HashMap;

// Easy way
pub fn word_count_one(string: &str) -> HashMap<String, u32> {
    let mut mapping = HashMap::new();
    for word in string.split_whitespace() {
        let word = word.to_lowercase()
            .to_string()
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect::<String>();
        if !word.is_empty() {
            *mapping.entry(word).or_insert(0) += 1;
        }
    }
    mapping
}

// More rustic way
pub fn word_count(string: &str) -> HashMap<String, u32> {
    string.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|c| !c.is_empty())
        .map(|c| c.to_string())
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}
