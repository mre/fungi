// takes an `FnMut` because used to take a variable in a closure (counter) and
// mutating (incrementing) it.
pub fn map<F, T, S>(input: Vec<T>, mut function: F) -> Vec<S>
where F: FnMut(T) -> S {
    let mut v = Vec::with_capacity(input.len());
    for item in input {
        v.push(function(item));
    }
    return v;
}
