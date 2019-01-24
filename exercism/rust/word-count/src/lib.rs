use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut c: HashMap<String, u32> = HashMap::new();
    
    // https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
    // for w in words.split_whitespace() {

    for w in words.split(|c: char| (c != '\'' && !c.is_alphanumeric()) || c.is_whitespace()) {
        let w = w.trim_matches(|c: char| !c.is_ascii_alphanumeric());
        if w.len() < 1 {
            continue;
        }

        *c.entry(w.to_lowercase()).or_insert(0) += 1;
    }
    return c;
}
