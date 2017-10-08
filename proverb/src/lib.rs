fn build_str(string: &str, next_string: &str) -> String {
    format!("For want of a {} the {} was lost.\n", string, next_string)
}

pub fn build_proverb(list: Vec<&str>) -> String {
    let mut resp = String::new();
    let empty = "";
    for (index, item) in list.iter().enumerate() {
        let next = &list.get(index + 1).unwrap_or(&empty);
        if next.is_empty() {
            match index {
                0 => resp.push_str("And all for the want of a nail."),
                1 => resp.push_str("And all for the want of a nail."),
                _ => resp.push_str("And all for the want of a horseshoe nail.")
            }
        }
        else {
            resp.push_str(&build_str(item, next));
        }
    }
    resp
}

pub fn build_proverb_2(list: Vec<&str>) -> String {
    let mut resp = String::new();
    let last = if list.len() < 3 {""} else {"horseshoe "};
    let iter = list.iter().peekable();

    for item in iter {
        if let Some(next_string) = iter.peek() {
            resp.push_str(&format!("For want of a {} the {} was lost.\n", item, next_string));
        } else {
            resp.push_str(&format!("And all for the want of a {}nail.", last));
        }
    }
    resp
}
