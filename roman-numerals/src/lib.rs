use std::convert::From;
use std::fmt;

static MAPPING: [(usize, &'static str); 13] = [
    (1, "I"), (4, "IV"), (5, "V"), (9, "IX"), (10, "X"), (40, "XL"), (50, "L"),
    (90, "XC"), (100, "C"), (400, "CD"), (500, "D"), (900, "CM"), (1000, "M")
];

pub struct Roman {
    num: usize,
}

impl From<usize> for Roman {
    fn from(num: usize) -> Self {
        Roman::new(num)
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut number = self.num;
        write!(f, "{}", MAPPING.iter().rev().fold(String::new(), |mut roman_string, &(value, string)| {
            while number >= value {
                roman_string.push_str(string);
                number -= value;
            }
            roman_string
        }))
    }
}

impl Roman {
    pub fn new(num: usize) -> Roman {
        Roman { num: num }
    }
}
