#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        _ if first_list == second_list => Comparison::Equal,
        (n, m) if n < m && is_sublist(first_list, second_list) => Comparison::Sublist,
        (n, m) if n > m && is_sublist(second_list, first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

pub fn is_sublist<T: PartialEq + std::fmt::Debug>(a: &[T], b: &[T]) -> bool {
    assert!(a.len() < b.len());
    if a.is_empty() {
        return true;
    }

    // b.windows(a.len()).any(|w| w == a)
    let mut i: usize = 0;
    let mut j: usize = a.len();
    while j <= b.len() {
        if &b[i..j] == a {
            return true;
        }
        
        i += 1;
        j += 1;
    }
    return false;
}
