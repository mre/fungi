use std::mem;

fn frep<'a>(f: &'a mut Vec<i32>) -> &'a Vec<i32> {
    let old_vec: Vec<i32> = mem::replace(f, vec![0, 0, 0]);
    println!("the old vec was {:?}", &old_vec);
    f
}

fn main() {
    let mut ft: Vec<i32> = vec![1, 2, 3];
    let ff: &Vec<i32> = frep(&mut ft);
    println!("Hello, world! {:?}", ff);
}
