use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();

    for a in 1..sum - 1 {
        for b in a..sum - a {
            let c = sum - b - a;
            if c <= b {
                break;
            }

            if (a as u64).pow(2) + (b as u64).pow(2) == (c as u64).pow(2) {
                result.insert([a, b, c]);
            }
        }
    }

    // but this is awesome:
    // https://github.com/exercism/rust/blob/5c35702d3494157b0ca069a9432b3ff700bdb1c5/exercises/pythagorean-triplet/example.rs
    // for a in 1..sum {
    //     /*
    //      * (where n is a one-character alias for sum)
    //      * c = n - a - b
    //      * a^2 + b^2 = c^2
    //      * a^2 + b^2 = n^2 - 2an - 2bn + a^2 + 2ab + b^2
    //      * 2bn - 2ab = n^2 - 2an
    //      * 2b(n - a) = n(n-2a)
    //      * b = n(n-2a) / 2(n-a)
    //      * b = (n(n-a) - an) / 2(n-a)
    //      */
    //     let b = sum / 2 - a * sum / (2 * (sum - a));
    //     if a >= b {
    //         break;
    //     }
    //     let c = sum - (a + b);
    //
    //     if a * a + b * b == c * c {
    //         triplets.insert([a, b, c]);
    //     }
    // }

    result
}
