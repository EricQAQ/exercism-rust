#[derive(Debug)]
struct Mapping {
    num: usize,
    text: String,
}

fn get_mapping() -> Vec<Mapping> {
    vec![
        Mapping { num: 3, text: "Pling".to_string() },
        Mapping { num: 5, text: "Plang".to_string() },
        Mapping { num: 7, text: "Plong".to_string() },
    ]
}

pub fn raindrops(n: usize) -> String {
    let mapping = get_mapping();
    let mut resp = String::new();
    for item in mapping.into_iter() {
        match n % item.num {
            0 => resp.push_str(&item.text),
            _ => (),
        };
    }
    if resp.is_empty() {
        resp.push_str(&n.to_string());
    }
    resp
}
