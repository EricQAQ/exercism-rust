use std::convert::Into;

#[derive(Debug)]
enum Caculate {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct WordProblem {
    command: String,
}

impl WordProblem {
    pub fn new<T: Into<String>>(command: T) -> Self {
        WordProblem { command: command.into() }
    }

    fn parse(&self) -> Vec<Caculate> {
        self.command.replace("?", "").to_lowercase()
            .split_whitespace()
            .fold(Vec::new(), |mut res, word| {
                match word.chars().all(|x| x.is_digit(10) || x == '-') {
                    true => {
                        res.push(Caculate::Number(word.parse::<i32>().unwrap()));
                        res
                    }
                    false => match word {
                        "plus" => { res.push(Caculate::Plus); res}
                        "minus" => { res.push(Caculate::Minus); res}
                        "multiplied" => { res.push(Caculate::Multiply); res}
                        "divided" => { res.push(Caculate::Divide); res }
                        _ => res
                    }
                }
            })
    }

    pub fn answer(&self) -> Result<i32, ()> {
        let stack = self.parse();
        if stack.len() < 2 || stack.len() % 2 == 0 { return Err(()) }
        let flag = stack.iter()
            .enumerate()
            .all(|(index, x)| match index % 2 {
                0 => match *x {
                    Caculate::Number(_) => true,
                    _ => false
                },
                _ => match *x {
                    Caculate::Divide | Caculate::Minus | Caculate::Multiply | Caculate::Plus => true,
                    _ => false
                }
            });
        match flag {
            false => Err(()),
            true => Ok(
                stack.iter()
                .fold((0, Caculate::Plus), |(sum, mut op), x| match *x {
                    Caculate::Number(a) => match op {
                        Caculate::Plus => (sum + a, op),
                        Caculate::Minus => (sum - a, op),
                        Caculate::Multiply => (sum * a, op),
                        Caculate::Divide => (sum / a, op),
                        _ => (sum / a, op),
                    },
                    Caculate::Plus => {
                        op = Caculate::Plus;
                        (sum, op)
                    }
                    Caculate::Minus => {
                        op = Caculate::Minus;
                        (sum, op)
                    }
                    Caculate::Divide => {
                        op = Caculate::Divide;
                        (sum, op)
                    }
                    Caculate::Multiply => {
                        op = Caculate::Multiply;
                        (sum, op)
                    }
                }).0
            )
        }
    }
}
