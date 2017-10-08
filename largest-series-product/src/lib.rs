use std::convert::Into;

pub fn lsp<T: Into<String>>(s: T, count: usize) -> Result<usize, ()> {
    if count == 0 {
        return Ok(1);
    }
    let string = s.into();
    if string.chars().any(|x| !x.is_digit(10)) || (string.len() < count) {
        return Err(());
    }
    match string
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .windows(count)
        .map(|w| w.into_iter().product())
        .collect::<Vec<usize>>()
        .iter()
        .max()
    {
        Some(&x) => Ok(x),
        None => Err(()),
    }
}
