/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // let re = regex::Regex::new(r"^[0-9]-?[0-9]{3}-?[0-9]{5}-?[0-9X]$").unwrap();
    // re.is_match(isbn) && isbn.chars()
    //     .filter(|&c| c != '-')
    //     .zip((1..=10).rev())
    //     .fold(0, |acc, (n, i)| acc + i * n.to_digit(10).unwrap_or(10) as usize) % 11 == 0
    let isbn = str::replace(isbn, "-", "");
    if isbn.len() != 10 {
        return false;
    }
    let mut checksum: u32 = 0;
    // https://doc.rust-lang.org/nightly/core/str/struct.CharIndices.html
    for (i, c) in isbn.char_indices() {
        if c.is_numeric() {
            checksum += c.to_digit(10).unwrap() * (10 - i as u32);
        } else if i == 9 && c == 'X' {
            checksum += 10;
        } else {
            return false;
        }
    }
    checksum % 11 == 0
}
