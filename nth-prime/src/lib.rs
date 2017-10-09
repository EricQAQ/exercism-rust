/// Pn <= nlnn+nlnlnn(n≥6)
fn get_upper_limit(num: &u32) -> f64 {
    let number: f64 = *num as f64;
    number * number.ln() + number * ((number.ln()).ln())
}

fn get_first_five_prime(num: u32) -> Result<usize, ()> {
    match num {
        1 => Ok(2),
        2 => Ok(3),
        3 => Ok(5),
        4 => Ok(7),
        5 => Ok(11),
        _ => Err(()),
    }
}

pub fn nth(num: u32) -> Result<usize, ()> {
    if num <= 5 { return get_first_five_prime(num) }
    else {
        // the upper number of nth prime
        let upper_limit = get_upper_limit(&num);
        // the total number count
        let total = (upper_limit.round() as u64 + 1) as usize;
        // sqrt number
        let sqrt = (upper_limit.sqrt().abs().round() as u64 + 1) as usize;
        let mut is_prime: Vec<bool> = Vec::with_capacity(total);
        let mut count = 0u32;
        for _ in 0..(total + 1) { is_prime.push(true); }

        for i in 2..sqrt + 1 {
            if is_prime[i] {
                let mut j = i * i;
                while j <= (upper_limit as usize) {
                    is_prime[j] = false;
                    j += i; 
                }
            }
        }
        for i in 2..(is_prime.len() + 1) {
            if is_prime[i] { count += 1; }
            if count == num { return Ok(i) }
        }
        return Err(())
    }
}
