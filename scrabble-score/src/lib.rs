use std::collections::HashMap;
use std::convert::Into;

fn get_mapping() -> HashMap<char, usize> {
    let mapping_vec: Vec<(char, usize)> = vec![
        ('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1), ('l', 1), ('n', 1),
        ('r', 1), ('s', 1), ('t', 1), ('d', 2), ('g', 2), ('b', 3), ('c', 3), 
        ('m', 3), ('p', 3), ('f', 4), ('h', 4), ('v', 4), ('w', 4), ('y', 4), 
        ('k', 5), ('j', 8), ('x', 8), ('q', 10), ('z', 10)
    ];
    mapping_vec.iter().fold(HashMap::new(), |mut mapping, &(chr, score)| {
        mapping.insert(chr, score);
        mapping
    })
}

pub fn score<T: Into<String>>(string: T) -> usize {
    let mapping = get_mapping();
    string.into().to_lowercase().chars().fold(0_usize, |score, chr| score + mapping.get(&chr).unwrap_or(&0))
}
