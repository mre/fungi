extern crate rand;

use rand::{thread_rng, Rng};
use std::ops::{Add, Sub};

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.len() <= 0 {
        return None;
    };
    if key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    };

    let mut result: Vec<char> = Vec::new();
    let kc: Vec<char> = key.chars().collect();

    // for (ch, k) in text.chars().zip(key.chars().cycle()) {
    for (i, c) in s.chars().enumerate() {
        // take the corresponding "key-char" to use
        let k: char = kc[i % key.len()];
        // get the "length of the shift" as the distance between
        // the "key-char" and the 'a'
        let shift: u8 = (k as u8) - b'a';
        // get the current char "position" in the alphabet, apply the shift,
        // wrap to the end, convert it back to a char.
        let cc: char = (b'a' + u8::add(c as u8 - b'a', shift) % 26) as char;
        result.push(cc);
    }
    return Some(result.iter().collect());
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.len() <= 0 {
        return None;
    };
    if key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    };

    let mut result: Vec<char> = Vec::new();
    let kc: Vec<char> = key.chars().collect();

    // for (ch, k) in text.chars().zip(key.chars().cycle()) {
    for (i, c) in s.chars().enumerate() {
        // take the corresponding "key-char" to use
        let k: char = kc[i % key.len()];
        // get the "length of the shift" as the distance between
        // the "key-char" and the 'a'
        let shift: u8 = (k as u8) - b'a';
        // get the current char "position" in the alphabet, apply the shift,
        // wrap to the end, convert it back to a char.
        let cc: char = (b'a' + u8::sub(26 + c as u8 - b'a', shift) % 26) as char;

        result.push(cc);
    }
    return Some(result.iter().collect());
}

fn key(length: usize) -> String {
    let mut rng = thread_rng();
    return (0..length)
        .map(|_| rng.gen_range(b'a', 1 + b'z') as char)
        .collect::<String>();
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = key(100);
    let ciphertext = encode(&key, s).expect("boom");
    (key, ciphertext)
}
