use std::collections::HashMap;

pub type Palindrome = u64;

fn reverse(n: u64) -> u64 {
    let mut m = n;
    let mut r = 0;
    while m > 0 {
        r *= 10;
        r += m % 10;
        m /= 10;
    }
    return r;
}

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let mut m: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();
    let mut n: u64;
    for i in min..=max {
        for j in i..=max {
            n = i * j;
            if n == reverse(n) {
                match m.get_mut(&n) {
                    Some(v) => {
                        v.push((i, j));
                    }
                    None => {
                        m.insert(n, vec![(i, j)]);
                    }
                }
            }
        }
    }
    let mut v: Vec<Palindrome> = Vec::new();
    for k in m.keys() {
        v.push(*k);
    }
    return v;
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    return palindromes.iter().min().cloned();
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    return palindromes.iter().max().cloned();
}
