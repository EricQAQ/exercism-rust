use std::convert::From;

pub struct Luhn {
    digits: Vec<u8>,
    length: usize,
}

impl Luhn {
    pub fn new(digits: Vec<u8>, length: usize) -> Self {
        Luhn { digits: digits, length: length }
    }

    pub fn is_valid(&self) -> bool {
        if self.length != self.digits.len() || self.digits.len() < 2 { return false }
        self.digits.iter()
            .enumerate()
            .fold(0, |result, (index, value)| {
                match index % 2 {
                    1 => match value * 2 > 9 {
                        true => result + value * 2 - 9,
                        false => result + value * 2
                    }
                    _ => result + value,
                }
            }) % 10 == 0
    }
}

impl From<String> for Luhn {
    fn from(string: String) -> Self {
        Luhn::new(
            string
                .replace(" ", "")
                .chars()
                .rev()
                .filter_map(|ch| match ch.to_digit(10) {
                    Some(value) => Some(value as u8),
                    None => None
                })
                .collect::<Vec<u8>>(),
            string
                .replace(" ", "")
                .chars()
                .collect::<String>()
                .len()
        )
    }
}

impl<'a> From<&'a str> for Luhn {
    fn from(string: &str) -> Self {
        Luhn::from(String::from(string))
    }
}

impl From<u8> for Luhn {
    fn from(value: u8) -> Self {
        Luhn::from(value.to_string())
    }
}

impl From<u16> for Luhn {
    fn from(value: u16) -> Self {
        Luhn::from(value.to_string())
    }
}

impl From<u32> for Luhn {
    fn from(value: u32) -> Self {
        Luhn::from(value.to_string())
    }
}

impl From<u64> for Luhn {
    fn from(value: u64) -> Self {
        Luhn::from(value.to_string())
    }
}

impl From<usize> for Luhn {
    fn from(value: usize) -> Self {
        Luhn::from(value.to_string())
    }
}
