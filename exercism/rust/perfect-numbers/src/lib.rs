#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    };
    let n: u64 = num;
    let limit = (n / 2) + 1;
    let sum: u64 = (1..limit).filter(|&x| n % x == 0).sum();

    Some(match sum {
        _ if sum == n => Classification::Perfect,
        _ if sum > n => Classification::Abundant,
        _ => Classification::Deficient,
    })

}
