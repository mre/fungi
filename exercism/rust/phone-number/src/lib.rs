// format:
//
// (NXX)-NXX-XXXX
// 
// where `N` is any digit from 2 through 9 and `X` is any digit from 0
// through 9.
// 
// Your task is to clean up differently formatted telephone numbers by
// removing punctuation and the country code (1) if present.
// 
pub fn number(user_number: &str) -> Option<String> {
    let mut n: Vec<char> = Vec::new();
    let mut i: usize = 0;
    for c in user_number.chars() {
        // ignore the prefix
        if i == 0 && c == '1' {
            continue;
        }
        // ignore non-numeric characters
        if !c.is_numeric() {
            continue;
        }
        // validate area code
        if i == 0 && c < '2' {
            return None;
        }
        // validate exchange code
        if i == 3 && c < '3' {
            return None;
        }
        // push subscriber number
        n.push(c);
        i += 1;
    }
    // validate length
    if n.len() != 10 {
        return None;
    }
    return Some(n.iter().collect::<String>());
}
