use std::collections::HashMap;
use std::collections::VecDeque;

// permutations without repetition:
//
// where n is the number of objects ((0..10).len())
// and r is the number of positions (chars.len())
// the total permutations are P(n,r) = n!/(n-r)!
//
// https://en.wikipedia.org/wiki/Permutation
// https://en.wikipedia.org/wiki/Heap's_algorithm
// Permutations by interchanges. B.R.Heap, The Computer Journal, 6(3) (1963)
// http://comjnl.oxfordjournals.org/content/6/3/293.full.pdf
//
// return permutations of size "size"
fn permute_with_limit<T, F: FnMut(&[T])>(
    size: usize,
    used: &mut Vec<T>,
    unused: &mut VecDeque<T>,
    action: &mut F,
) {
    if unused.is_empty() || used.len() == size {
        action(used);
    } else {
        for _ in 0..unused.len() {
            used.push(unused.pop_front().unwrap());
            permute_with_limit(size, used, unused, action);
            unused.push_back(used.pop().unwrap());
        }
    }
}

// "I + BB == ILL"
// [('I', 1), ('B', 9), ('L', 0)]
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    println!("solving \"{}\"", input);
    // prepare the result
    // let result: HashMap<char, u8> = HashMap::new();
    // prepare the result that must be in the same order than the input
    let mut results: Vec<HashMap<char, u8>> = Vec::new();
    // filter from the input string all the alphabetic characters
    let mut chars: Vec<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();
    // sort and remove the duplicates
    chars.sort();
    chars.dedup();
    // we have 10 possible values for each char (0..10 digits)
    let n: usize = 10;
    // and there are r positions to fill (Permutations: P(n,r))
    let r: usize = chars.len();
    // the list of possible values
    let mut queue: VecDeque<u8> = (0..n as u8).collect::<VecDeque<u8>>();
    // the "matrix" of all possible permutations P(n,r) = n!/(n-r)!
    let mut mtrix: Vec<Vec<u8>> = Vec::new();
    // collecting all the permutations with the heap's algorithm
    permute_with_limit(r, &mut Vec::new(), &mut queue, &mut |perm| {
        mtrix.push(perm.to_vec())
    });
    // let's start to brute-force this input.
    // for each permutation
    for (_i, v) in mtrix.iter().enumerate() {
        // create a vector of pairs (char, value)
        let z: Vec<(&char, &u8)> = chars.iter().zip(v).collect::<Vec<(&char, &u8)>>();
        // transform this vector of pairs in a lookup table (hashmap)
        let mut m: HashMap<char, u8> = HashMap::new();
        for (&k, &v) in z.iter() {
            m.insert(k, v);
        }
        // prepare the "converted" input as a vector of charactes but
        // replacing the "letters" with "digits"
        let mut digits: Vec<char> = Vec::with_capacity(input.len());
        use std::char;
        for c in input.chars() {
            // pushing the converted character if is a digit
            digits.push(match m.get(&c) {
                Some(&d) => {
                    // check the digit boundaries
                    assert!(d < 10);
                    // convert the char into its digit representation
                    // char::from_digit(d as u32, 10).expect("boom")
                    (d as u8 + b'0') as char
                }
                // if it's a symbol just push it as is
                None => c,
            });
        }
        // here the digits is the "string" representation of a simple
        // equation that we have to parse and verify
        let equation: String = digits.iter().collect::<String>();
        // split left side and right side of the equal sign
        let terms: Vec<&str> = equation
            .split_whitespace()
            .filter(|x| x != &"+" && x != &"==")
            .collect::<Vec<&str>>();
        // the left side
        let addends = &terms[0..terms.len() - 1];
        // leading zero numbers are invalid
        if addends.iter().any(|n| n.starts_with('0')) {
            continue;
        }
        // the right side after the equal sign
        let res = terms.last().expect("boom");
        // leading zero numbers are invalid        
        if res.starts_with('0') {
            continue;
        }
        // fold with add and compare
        if addends
            .iter()
            .map(|n| str::parse::<u32>(n))
            .try_fold(0, |acc, x| x.map(|y| acc + y))
            .expect("boom")
            == str::parse::<u32>(res).expect("boom")
        {
            // found one!
            println!("found: {:?}", m);
            results.push(m);
        }
    }
    if results.len() == 0 {
        return None;
    }
    return Some(results[0].to_owned());
}
