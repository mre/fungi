pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let mut out = 1;
    for _ in 0..a {
        out = (out * g) % p
    }
    out
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    public_key(p, b_pub, a)
}

#[allow(dead_code)]
// faster modular exponentiation.
// modpow(g, a, p)
fn modpow(x: u64, e: u64, m: u64) -> u64 {
    (0..e).fold(1, |a, _| (a * x) % m)
}
