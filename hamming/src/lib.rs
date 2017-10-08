use std::convert::Into;

pub fn hamming_distance<T: Into<String>>(s1: T, s2: T) -> Result<u32, ()> {
    let (s1, s2) = (s1.into(), s2.into());
    if s1.len() != s2.len() { return Err(()) }
    Ok(s1.char_indices().fold(0, |acc, (index, c)| {
        match c == s2.chars().nth(index).unwrap() {
            true => acc,
            false => acc + 1,
        }
    }))
}

pub fn hamming_distance_two<'a, T: Into<&'a str>>(s1: T, s2: T) -> Result<u32, ()> {
    let (s1, s2) = (s1.into(), s2.into());
    if s1.len() != s2.len() { return Err(()) }
    Ok(s1.chars().zip(s2.chars()).filter(|&(c1, c2)| c1 != c2).count() as u32)
}
