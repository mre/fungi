fn is_prime(n: u32) -> bool {
    !(2..n - 1).any(|i| n % i == 0)
}

fn nth_m(n: u32) -> Option<u32> {
    (2..).filter(|&i| is_prime(i)).nth((n) as usize)
}

pub fn nth(n: u32) -> u32 {
    match nth_m(n) {
        None => 0,
        Some(x) => x,
    }
}
