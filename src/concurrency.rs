// https://doc.rust-lang.org/stable/book/second-edition/ch16-00-concurrency.html

use std::thread;

fn one() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    // stop the main thread here
    // let _ = handle.join();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    let _ = handle.join();

}

// Using move Closures with Threads
fn two() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    let _ = handle.join();
}

pub fn sample() {
    one();
    two();
}
