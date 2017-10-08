use std::collections::HashMap;

pub fn nucleotide_counts(string: &str) -> Result<HashMap<char, usize>, ()> {
    let choices = vec!['A', 'C', 'G', 'T'];
    let mut mapping: HashMap<char, usize> = choices.iter().fold(HashMap::new(), |mut map, ch| {
        map.insert(*ch, 0);
        map
    });
    // or like this
    // let mut mapping: HashMap<char, usize> = choices.iter().map(|x| (*x, 0)).collect();
    for chr in string.to_string().chars() {
        if choices.contains(&chr) {
            *mapping.get_mut(&chr).unwrap() += 1;
        } else {
            return Err(());
        }
    }
    Ok(mapping)
}

pub fn count(ch: char, string: &str) -> Result<usize, ()> {
    let choices = vec!['A', 'C', 'G', 'T'];
    if !choices.contains(&ch) {
        return Err(());
    }
    string
        .to_string()
        .chars()
        .fold(Ok(0), move |total, chr| match choices.contains(&chr) {
            false => return Err(()),
            true => match ch == chr {
                false => total,
                true => Ok(total.unwrap() + 1),
            },
        })
}
