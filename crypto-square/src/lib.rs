use std::cmp;
use std::cmp::Ordering;

fn get_column_and_row(c: u64, r: u64, result: &u64) -> (usize, usize) {
    match (c * r).cmp(result) {
        Ordering::Equal => (c as usize, r as usize),
        Ordering::Greater => (c as usize, (r - 1) as usize),
        Ordering::Less => get_column_and_row(c + 1, r + 1, result)
    }
}

fn get_string_vec(string: &String, row: &usize, column: &usize) -> Vec<Vec<char>> {
    let mut rest_str = &*string.clone();
    (0..*row).fold(Vec::new(), move |mut acc, _| {
        let (chunk, rest) = rest_str.split_at(cmp::min(*column, rest_str.len()));
        acc.push(chunk.chars().collect::<Vec<char>>());
        rest_str = rest;
        acc
    })
}

fn get_result_vec(string_vec: Vec<Vec<char>>, row: &usize, column: &usize) -> String {
    let v: Vec<String> = vec![String::with_capacity(*row); *column];
    (0..*column).into_iter().fold(v, |mut acc, c| {
        for r in 0..*row {
            if let Some(&value) = string_vec[r].get(c) {
                acc[c].push(value);
            }
        }
        acc
    }).join(" ")
}

pub fn encrypt(s: &str) -> String {
    let string: String = s
        .to_lowercase()
        .split(|ch: char| !ch.is_alphabetic() )
        .collect::<String>();
    let sqrt = (string.len() as f64).sqrt().round() as u64;
    let (column, row) =get_column_and_row(sqrt, sqrt, &(string.len() as u64));
    let string_vector = get_string_vec(&string, &row, &column);
    get_result_vec(string_vector, &row, &column)
}
