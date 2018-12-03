pub fn series(digits: &str, len: usize) -> Vec<String> {
    // For example, the string "49142" has the following 3-digit series:
    // "491"
    // "914"
    // "142"
    //
    // alternative:
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.windows
    (0..digits.len() + 1 - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}
