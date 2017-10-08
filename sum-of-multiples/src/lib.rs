use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter().fold(HashSet::new(), |mut set, x| {
        let mut num = 1;
        while num * x < limit {
            set.insert(num * x);
            num += 1;
        }
        set
    }).iter().sum()
}
