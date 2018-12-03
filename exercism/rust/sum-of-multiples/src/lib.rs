pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut total = 0;
    for i in 1..limit {
        if factors.iter().any(|&x| x != 0 && i % x == 0) {
            total += i
        }
    }
    total
}
