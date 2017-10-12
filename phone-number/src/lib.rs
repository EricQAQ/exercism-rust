pub fn number(s: &str) -> Option<String> {
    let phone = s.chars()
        .filter(|&x| x.is_digit(10))
        .collect::<String>();
    match phone.len() {
        10 => match (phone.chars().nth(0).unwrap().to_digit(10).unwrap(),
                     phone.chars().nth(3).unwrap().to_digit(10).unwrap()) {
            (0...1, _) => None,
            (_, 0...1) => None,
            (_, _) => Some(phone.to_string()),
        },
        11 => match (phone.chars().nth(0).unwrap().to_digit(10).unwrap(),
                     phone.chars().nth(1).unwrap().to_digit(10).unwrap(),
                     phone.chars().nth(4).unwrap().to_digit(10).unwrap()) {
            (0, _, _) => None,
            (2...9, _, _) => None,
            (_, 0...1, _) => None,
            (_, _, 0...1) => None,
            _ => Some(phone[1..].to_string()),
        },
        _ => None
    }
}
