pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1),
        n => collatz(3 * n + 1).map(|x| x + 1),
    }

    // insanely expensive:
    // if n == 0 { return None; }
    // let mut count = 0;
    // let mut num = n;
    // while n > 1 {
    //     count += 1;
    //     if num % 2 == 0 {
    //         num /= 2;
    //     } else {
    //         num = 3 * num + 1;
    //     }
    // };
    // Some(count)
}
