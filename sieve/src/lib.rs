pub fn primes_up_to(limit: usize) -> Vec<usize> {
    let mut result_int = (1..limit).map(|x| x + 1).collect::<Vec<usize>>();
    let mut pivot = Some(2);

    while let Some(value) = pivot {
        result_int.retain(|&x| x == value || x % value != 0);
        pivot = result_int.clone().into_iter().find(|&x| x > value);
    }
    result_int
}
