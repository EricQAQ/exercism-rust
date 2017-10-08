pub fn square(s: u32) -> u64 {
    match s {
        0 => panic!("Square must be between 1 and 64"),
        65 => panic!("Square must be between 1 and 64"),
        _ => 2u64.pow(s-1),
    }
}

pub fn total() -> u64 {
    (1..65).fold(0, |acc, x| acc + square(x))
}
