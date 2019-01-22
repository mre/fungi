use std::collections::HashSet;
use std::iter::FromIterator;

/// Determine whether a sentence is a pangram.
const LEN: usize = 26;

pub fn is_pangram(sentence: &str) -> bool {
    if sentence.len() < LEN {
        return false;
    }
    
    let a:  HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    let mut s: HashSet<char> = HashSet::new();
    for c in sentence.to_lowercase().chars() {
        s.insert(c);
    }
    return a.iter().all(|v| s.contains(v));
}
