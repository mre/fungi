use std::collections::HashMap;

static NUCLEOTIDES: [char; 4] = ['A', 'T', 'G', 'C'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if NUCLEOTIDES.contains(&nucleotide) {
        let (res, err): (Vec<char>, Vec<char>) =
            dna.chars().into_iter().partition(|ref c| NUCLEOTIDES.contains(c));
        if err.len() > 0 {
            return Err(err[0]);
        }
        return Ok(
            res.iter().fold(0, |sum, c| if *c == nucleotide { sum + 1 } else { sum }));
    }
    return Err(nucleotide);
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut cnt = HashMap::new();
    cnt.insert('A', 0);
    cnt.insert('C', 0);
    cnt.insert('G', 0);
    cnt.insert('T', 0);

    for c in dna.chars() {
        if cnt.contains_key(&c) {
            *cnt.entry(c).or_insert(0) += 1;
        } else {
            return Err(c);
        }
    }
    return Ok(cnt);
}
