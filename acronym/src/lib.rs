use std::ascii::AsciiExt;

fn handle_camel(camel_string: &str) -> String {
    camel_string
        .char_indices()
        .fold(String::new(), |mut string, (index, chr)| {
            if index == 0 {
                string.push(chr.to_ascii_uppercase());
            } else if index != 0
                && camel_string.chars().nth(index - 1).unwrap().is_lowercase()
                && chr.is_uppercase()
            {
                string.push(chr.to_ascii_uppercase());
            }
            string
        })
}

pub fn abbreviate(string: &str) -> String {
    string
        .split(|ch: char| !ch.is_alphabetic())
        .filter(|ch| !ch.is_empty())
        .collect::<Vec<&str>>()
        .iter()
        .fold(String::new(), |mut resp, &st| {
            resp.push_str(&handle_camel(&st));
            resp
        })
}
