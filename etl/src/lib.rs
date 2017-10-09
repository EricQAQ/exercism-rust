use std::collections::BTreeMap;
use std::ascii::AsciiExt;

pub fn transform_one(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input.iter().fold(BTreeMap::new(), |mut mapping, (&key, ref list)| {
        for &v in list.iter() {
            mapping.insert(v.to_ascii_lowercase(), key);
        }
        mapping
    })
}

// More Rustic Way!
pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input.iter()
        .flat_map(|(&score, chars)| {
            chars.iter()
                .map(move |ch| (ch.to_ascii_lowercase(), score))
        })
        .collect()
}
