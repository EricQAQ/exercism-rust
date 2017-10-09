pub fn map_function(values: Vec<i32>, func: &Fn(i32) -> i32) -> Vec<i32> {
    values.iter()
        .map(|&v| func(v))
        .collect()
}

pub fn map_closure<F>(values: Vec<i32>, func: F) -> Vec<i32>
    where F: Fn(i32) -> i32 {
    values.iter()
        .map(|&v| func(v))
        .collect::<Vec<i32>>()
}
