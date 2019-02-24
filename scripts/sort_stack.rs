fn main() -> () {
    let mut stack: Vec<usize> = vec![9, 3, 2, 4, 1, 5, 4, 3, 8, 7];
    println!("{:?}", stack);
    let mut ordrd: Vec<usize> = Vec::new();
    
    while !stack.is_empty() {
        // pop the next to sort
        let tmp: usize = stack.pop().expect("nothing to pop");
        // peek
        let mut top: Option<&usize> = ordrd.last();
        while top.is_some() && top.expect("nothing to peek") > &tmp {
            let p: usize = ordrd.pop().unwrap();
            // push back
            stack.push(p);
            top = ordrd.last();
        }
        // push sorted
        ordrd.push(tmp);
    }
    println!("{:?}", ordrd);
}
