pub fn find<'a, T: Ord>(array: &'a [T], key: T) -> Option<usize> {
    use std::cmp::Ordering;

    let mut a = array;
    let mut b = 0;
    while !a.is_empty() {
        let m = (a.len() - 1) / 2;
        match key.cmp(&a[m]) {
            Ordering::Equal  => return Some(b + m),
            Ordering::Less => a = a.split_at(m).0,
            Ordering::Greater => {
                b += m + 1;
                a = a.split_at(m + 1).1
            }
        };
    }

    None
}

