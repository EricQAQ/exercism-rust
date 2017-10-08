/// a = 1000*(500-b) / (1000-b)
// pub fn find() -> Option<usize> {
//     for b in (1..334) {
//         let a = 1000 * (500 - b) / (1000 - b);
//         let c_s: f64 = (a * a + b * b) as u64 as f64;
//         let c = (c_s).sqrt() as usize;
//         if a + b + c == 1000 {
//             return Some(a * b * c)
//         }
//     }
//     None
// }

// More rustic method
pub fn find() -> Option<usize> {
    (1..334).into_iter().fold(None, |acc, b| {
        if let Some(_) = acc { return acc; }
        let a = 1000 * (500 - b) / (1000 - b);
        let c_s: f64 = (a * a + b * b) as u64 as f64;
        let c = (c_s).sqrt() as usize;
        if a + b + c == 1000 {
            return Some(a * b * c);
        }
        acc
    })
}

// Another rustic way
// pub fn find() -> Option<usize> {
//     let mut triplet = Vec::with_capacity(3);
//     (1..334).into_iter().find(|&b| {
//         let a = 1000 * (500 - b) / (1000 - b);
//         let c_s: f64 = (a * a + b * b) as u64 as f64;
//         let c = (c_s).sqrt() as usize;
//         if a + b + c == 1000 {
//             triplet.push(a);
//             triplet.push(b);
//             triplet.push(c);
//             return true
//         }
//         false
//     });
//     if triplet.len() == 3 {
//         return Some(triplet.iter().fold(1, |acc, x| acc * x))
//     }
//     None
// }
