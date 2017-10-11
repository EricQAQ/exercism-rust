pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        let string = self.replace(" ", "");
        println!("{}", string);
        if string.chars().any(|ch| !ch.is_digit(10)) || string.len() < 2 {
            return false
        }
        string
            .chars()
            .rev()
            .map(|ch| ch.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
            .iter()
            .enumerate()
            .fold(0, |acc, (index, value)| match index % 2 {
                1 => match value * 2 > 9 {
                    true => acc + value * 2 - 9,
                    false => acc + value * 2,
                },
                _ => acc + value,
            }) % 10 == 0
    }
}

impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        println!("{}", self.to_string().valid_luhn());
        self.to_string().valid_luhn()
        // String::from(*self).valid_luhn()
    }
}

impl Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}
