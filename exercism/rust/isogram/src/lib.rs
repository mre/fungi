use std::collections::HashSet;

pub fn check_hashset(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))
}

pub fn check(query: &str) -> bool {
    let mut target = query.to_lowercase().replace(' ', "").replace('-', "");
    while let Some(ch) = target.pop() {
        if target.find(ch) != None {
            return false;
        }
    }
    true
}
