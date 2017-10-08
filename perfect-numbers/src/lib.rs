use std::collections::HashSet;
use std::cmp;

#[derive(Debug, PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient,
}


pub fn classify_one<'a>(number: u64) -> Result<Classification, &'a str> {
    if number == 0 { return Err("Number must be positive") }
    let half = number / 2 + 1;
    let result = (1..half + 1).fold(HashSet::new(), |mut acc, item| {
        if number % item == 0 && item < number {
            acc.insert(item);
            if number / item < number {
                acc.insert(number / item);
            }
        }
        acc
    });
    if result.is_empty() {
        return Ok(Classification::Deficient)
    }
    let all_sum: u64 = result.into_iter().sum();
    if all_sum == number { return Ok(Classification::Perfect)}
    else if all_sum > number { return Ok(Classification::Abundant)}
    else { return Ok(Classification::Deficient)}
}

pub fn classify<'a>(number: u64) -> Result<Classification, &'a str> {
    if number == 0 { return Err("Number must be positive") }

    match number.cmp(&(1..(number / 2 + 1)).filter(|i| number % i == 0).sum()) {
        cmp::Ordering::Equal => Ok(Classification::Perfect),
        cmp::Ordering::Less => Ok(Classification::Abundant),
        cmp::Ordering::Greater => Ok(Classification::Deficient),
    }
}
