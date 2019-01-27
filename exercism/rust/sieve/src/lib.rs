pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve: Vec<bool> = (0..=upper_bound).map(|i| i > 1).collect();

    for n in 0..=upper_bound {
        if sieve[n as usize] {
            let mut multiple = n + n;
            while multiple <= upper_bound {
                sieve[multiple as usize] = false;
                multiple += n;
            }
        }
    }
    return sieve
        .iter()
        .enumerate()
        .filter_map(|(n, t)| if *t { Some(n as u64) } else { None })
        .collect();
}
