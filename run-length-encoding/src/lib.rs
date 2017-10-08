use std::convert::Into;

pub fn decode<T: Into<String>>(s: T) -> String {
    let mut resp = String::new();
    let mut count = String::new();
    let string = s.into();
    for item in string.chars() {
        if item.is_numeric() {
            count.push(item);
        } else {
            let num = count.parse::<usize>().unwrap_or(1);
            resp.push_str(&item.to_string().repeat(num));
            count.clear();
        }
    }
    resp
}

pub fn encode<T: Into<String>>(s: T) -> String {
    let mut resp = String::new();
    let string = s.into();
    let mut flag = String::new();
    let mut count = 1;
    for item in string.chars() {
        if flag.is_empty() {
            flag.push(item);
        } else if item.to_string() == flag {
            count += 1;
        } else {
            if count != 1 {
                resp.push_str(&count.to_string());
            }
            resp.push_str(&flag);
            flag.clear();
            flag.push(item);
            count = 1;
        }
    }
    if count != 1 {
        resp.push_str(&count.to_string());
    }
    resp.push_str(&flag);
    resp
}
