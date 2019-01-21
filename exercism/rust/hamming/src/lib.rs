/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    // s1.chars().zip(s2.chars()).filter(|&(a, b)| a != b).count()
    let mut d: usize = 0;
    let s1_b = s1.as_bytes();
    let s2_b = s2.as_bytes();
    for i in 0..s1.len() {
        d += if s1_b[i] == s2_b[i] { 0 } else { 1 };
    }
    return Some(d);
}
