use std::convert::From;
use std::convert::Into;

static BRACKETS: [char; 6] = ['[', ']', '(', ')', '{', '}'];

pub struct Brackets {
    content: String,
}

impl<'a> From<&'a str> for Brackets {
    fn from(string: &str) -> Brackets {
        Brackets::new(string)
    }
}

impl Brackets {
    pub fn new<T: Into<String>>(string: T) -> Self {
        Brackets { content: string.into() }
    }

    fn is_pairs(pre_bracket: char, post_bracket: char) -> bool {
        println!("{}, {}", pre_bracket, post_bracket);
        match (pre_bracket, post_bracket) {
            ('[', ']') => true,
            ('(', ')') => true,
            ('{', '}') => true,
            _          => false,
        }
    }
    
    // More Rustic Way!
    pub fn are_balanced(&self) -> bool {
        let (resp, flag) = self.content.chars()
            .filter(|ch| BRACKETS.iter().any(|x| x == ch))
            .fold((Vec::new(), true), |(mut acc, mut flag), ch| match ch {
                '[' | '(' | '{' => { acc.push(ch); (acc, flag) }
                ']' | ')' | '}' => match acc.pop() {
                    Some(b) => (acc, Brackets::is_pairs(b, ch)),
                    None => (acc, false)
                }
                _ => { flag = false; return (acc, flag) }
            });
        flag && resp.is_empty()
    }

    pub fn are_balanced_one(&self) -> bool {
        let bracks = self.content.chars()
            .filter(|ch| BRACKETS.iter().any(|x| x == ch))
            .collect::<Vec<char>>();
        let mut resp = Vec::new();
        for &bracket in bracks.iter() {
            if bracket == '[' || bracket == '(' || bracket == '{' {
                resp.push(bracket);
            }
            else if bracket == ']' {
                if let Some(b) = resp.pop() {
                    if b != '[' { return false }
                }
                else { return false }
            }
            else if bracket == ')' {
                if let Some(b) = resp.pop() {
                    if b != '(' { return false }
                }
                else { return false }
            }
            else if bracket == '}' {
                if let Some(b) = resp.pop() {
                    if b != '{' { return false }
                }
                else { return false }
            }
        }
        resp.is_empty()
    }
}
