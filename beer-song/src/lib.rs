
pub fn verse(n: i32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1).to_string()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut resp = String::new();
    for num in (end..start + 1).rev() {
        resp.push_str(&verse(num));
        resp.push_str("\n");
        if num == end {
            resp.pop();
        }
    }
    resp
}
