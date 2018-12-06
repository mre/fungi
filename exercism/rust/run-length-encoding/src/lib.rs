pub fn encode(source: &str) -> String {
    let mut result: String = String::new();
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable
    // fn peekable(self) -> Peekable<Self>
    // Creates an iterator which can use peek to look at the next element of the iterator without consuming it.
    // Adds a peek method to an iterator.
    //
    // https://doc.rust-lang.org/std/iter/struct.Peekable.html#method.peek
    // pub fn peek(&mut self) -> Option<&<I as Iterator>::Item>
    // Returns a reference to the next() value without advancing the iterator.
    let mut chars = source.chars().peekable();
    let mut count = 0;
    while let Some(curr) = chars.next() {
        count += 1;
        // if curr is the last of the "streak", we close the count
        if chars.peek() != Some(&curr) {
            if count > 1 {
                result.push_str(&count.to_string())
            }
            result.push(curr);
            count = 0;
        }
    }
    return result;
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            count.push(c)
        } else {
            result.push_str(&c.to_string().repeat(count.parse::<usize>().unwrap_or(1)));
            count.clear()
        }
    }
    result
}
