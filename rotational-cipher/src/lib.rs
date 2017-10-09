fn cipher(ch: char, rot: &usize) -> char {
    let mut lower_iter = ('a' as u8..('z' as u8) + 1).cycle();
    let mut upper_iter = ('A' as u8..('Z' as u8) + 1).cycle();
    match ch.is_digit(10) || !ch.is_alphanumeric() {
        true => ch,
        false => match ch.is_uppercase() {
            false => {
                lower_iter.position(|x| x == (ch as u8));
                lower_iter.nth(*rot - 1).unwrap() as char
            }
            // true => upper_iter.nth(lower_iter.clone().position(|x| x == (ch as u8)).unwrap() + *rot).unwrap() as char
            true => {
                upper_iter.position(|x| x == (ch as u8));
                upper_iter.nth(*rot - 1).unwrap() as char
            }
        }
    }
}

pub fn rotate(string: &str, rot: usize) -> String {
    if rot == 0 { return string.to_string()}
    string
        .chars()
        .map(|x| cipher(x, &rot))
        .collect::<String>()
}
