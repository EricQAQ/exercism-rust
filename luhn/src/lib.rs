use std::convert::Into;

pub fn is_valid_one<T: Into<String>>(s: T) -> bool {
    let string = s.into().trim().to_string();
    if string.len() < 2 { return false }
    let mut all_sum =  0_usize;
    let mut index = 1;
    for chr in string.chars().rev() {
        if chr == ' ' { continue; }
        if !chr.is_numeric() { return false }
        let x = chr.to_string().parse::<usize>().unwrap();
        match index % 2 {
            0 => match x * 2 > 9 {
                false => all_sum += x * 2,
                true => all_sum += x * 2 - 9,
            },
            _ => all_sum += x
        };
        index += 1; 
    }
    match all_sum % 10 {
        0 => true,
        _ => false
    }
}

pub fn is_valid<T: Into<String>>(s: T) -> bool {
    let string = s.into().replace(" ", "").chars().rev().collect::<String>();
    if string.chars().any(|x| !x.is_digit(10)) { return false }
    else if string.len() < 2 { return false }
    match string.char_indices().fold(0, |acc, (i, chr)| {
        let x = chr.to_string().parse::<usize>().unwrap();
        match i % 2 {
            1 => match x * 2 > 9 {
                false => acc + x * 2,
                true => acc + x * 2 - 9,
            },
            _ => acc + x
        }
    }) % 10 {
        0 => true,
        _ => false
    }
}
