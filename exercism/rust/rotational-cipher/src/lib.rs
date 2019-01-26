pub fn rotate(input: &str, key: i8) -> String {
    let mut rotated: String = String::with_capacity(input.len());
    for c in input.chars() {
        rotated.push(match c {
            'a'...'z' => (b'a' as u8 + ((c as u8 - b'a') + key as u8) % 26) as char,
            'A'...'Z' => (b'A' as u8 + ((c as u8 - b'A') + key as u8) % 26) as char,
            _ => c,
        });
    }
    return rotated;
}
