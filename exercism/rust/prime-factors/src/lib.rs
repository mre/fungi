pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut fs = Vec::new();
    for f in (2..).take_while(|i| i * i <= n) {
        while num % f == 0 {
            fs.push(f);
            num /= f;
        }
    }
    if num > 1 {
        fs.push(num);
    }
    fs
}
