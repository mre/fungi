pub fn abbreviate(phrase: &str) -> String {
    let mut s: Vec<char> = Vec::new();
    let mut u: bool = false;
    let mut p: char = ' ';
    for c in phrase.chars() {
        if p.is_uppercase() {
            p = c;
            continue;
        }
        p = c;
        if !u && c.is_uppercase() {
            s.push(c);
            continue;
        }
        if c.is_whitespace() || !c.is_alphabetic() {
            u = true;
            continue;
        }
        if u {
            s.push(c.to_ascii_uppercase());
            u = false;
        }
    }
    return s.iter().collect::<String>();

    // alternatively
    // phrase.windows(2).filter(|pair| {
    //         let prev = pair[0];
    //         let curr = pair[1];
    //         (curr.is_uppercase() && !prev_char.is_uppercase()) ||
    //         (curr.is_lowercase() && !prev_char.is_alphabetic())
    //     })
    //     .map(|pair| pair[1])
    //     .collect::<String>()
    //     .to_uppercase()
}
