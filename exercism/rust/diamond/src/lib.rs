pub fn get_diamond(c: char) -> Vec<String> {
    let z: u8 = (c as u8) - b'A' + 1; // character 1-based position
    let mut upper: Vec<String> = Vec::with_capacity((z as usize) *2 -1);
    for i in 0..z {
        let mut left: Vec<char> = Vec::with_capacity(z as usize);
        for _ in 0..z {
            left.push(' ');
        }
        left[i as usize] = (b'A' + i) as char;
        let mut right: Vec<char> = left.clone();
        right.reverse();
        for j in left.iter().skip(1) {
            right.push(*j);
        }
        upper.push(right.iter().collect::<String>());
    }
    let mut lower: Vec<String> = upper.clone();
    lower.reverse();
    for j in lower.iter().skip(1) {
        upper.push(j.clone());
    }
    return upper;
}

