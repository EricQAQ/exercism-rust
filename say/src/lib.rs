//
// See Rust Language Specific Instructions
// below normal exercise description.
//
use std::collections::HashMap;

fn get_mapping() -> HashMap<u64, String> {
    let list = vec![
        (0, "zero".to_string()),
        (1, "one".to_string()), (2, "two".to_string()),
        (3, "three".to_string()), (4, "four".to_string()),
        (5, "five".to_string()), (6, "six".to_string()),
        (7, "seven".to_string()), (8, "eight".to_string()),
        (9, "nine".to_string()), (10, "ten".to_string()),
        (11, "eleven".to_string()), (12, "twelve".to_string()),
        (13, "thirteen".to_string()), (14, "fourteen".to_string()),
        (15, "fifteen".to_string()), (16, "sixteen".to_string()),
        (17, "seventeen".to_string()), (18, "eighteen".to_string()),
        (19, "nineteen".to_string()), (20, "twenty".to_string()),
        (30, "thirty".to_string()), (40, "forty".to_string()),
        (50, "fifty".to_string()), (60, "sixty".to_string()),
        (70, "seventy".to_string()), (80, "eighty".to_string()),
        (90, "ninety".to_string()), (100, "hundred".to_string()),
        (1000, "thousand".to_string()), (1_000_000, "million".to_string()),
        (1_000_000_000, "billion".to_string()), (1_000_000_000_000, "trillion".to_string()),
        (1_000_000_000_000_000, "quadrillion".to_string()),
        (1_000_000_000_000_000_000, "quintillion".to_string()),
    ];
    list.into_iter().fold(HashMap::new(), |mut acc, (x, y)| {
        acc.insert(x, y);
        acc
    })
}

fn make_string(number: &u64, resp: &mut String, mapping: &HashMap<u64, String>) {
    if *number == 0 {
        if resp.is_empty() {
            resp.push_str(&*mapping.get(number).unwrap());
            return
        } else { return }
    } else if *number <= 20{
        resp.push_str(&*mapping.get(number).unwrap());
        return
    } else if *number < 100 {
        let rev = *number % 10;
        let times = *number - rev;
        resp.push_str(&mapping.get(&times).unwrap());
        resp.push_str("-");
        resp.push_str(&mapping.get(&rev).unwrap());
        return
    } else if *number < 1000 {
        let rev = *number % 100;
        let times = (*number - rev) / 100;
        resp.push_str(&*mapping.get(&times).unwrap());
        resp.push_str(" hundred ");
        make_string(&rev, resp, mapping);
    } else if *number < 1_000_000 {
        let rev = *number % 1000;
        let times = (*number - rev) / 1000;
        make_string(&times, resp, mapping);
        resp.push_str(" thousand ");
        make_string(&rev, resp, mapping);
    } else if *number < 1_000_000_000 {
        let rev = *number % 1_000_000;
        let times = (*number - rev) / 1_000_000;
        make_string(&times, resp, mapping);
        resp.push_str(" million ");
        make_string(&rev, resp, mapping);
    } else {
        let rev = *number % 1_000_000_000;
        let times = (*number - rev) / 1_000_000_000;
        make_string(&times, resp, mapping);
        resp.push_str(" billion ");
        make_string(&rev, resp, mapping);
    }
}

pub fn encode(number: u64) -> String {
    let mut resp = String::new();
    let mapping = get_mapping();
    make_string(&number, &mut resp, &mapping);
    resp.trim().to_string()
}
