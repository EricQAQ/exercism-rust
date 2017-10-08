fn find_prime(factor_list: &mut Vec<usize>, number: &usize) {
    if *number == 1 { return }
    let mut num = *number;
    for i in (2..&num + 1) {
        while num != i {
            if num % i == 0 {
                factor_list.push(i);
                num = num / i;
                return find_prime(factor_list, &num)
            }
            else { break; }
        }
        if num == i {
            factor_list.push(i);
            return
        }
        else if i > num { return }
    }
}


pub fn factors(number: usize) -> Vec<usize> {
    let mut factor_list: Vec<usize> = Vec::new();
    if number == 1 { return factor_list}
    find_prime(&mut factor_list, &number);
    factor_list
}
