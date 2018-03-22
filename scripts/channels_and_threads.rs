use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

fn channel() {
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = tx.clone();
        // Each thread will send its id via the channel
        thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(id).unwrap();
            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });
    }
    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(rx.recv());
    }
    // Show the order in which the messages were sent
    println!("{:?}", ids);
}

// https://rustbyexample.com/std_misc/channels.html

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];
    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

// https://rustbyexample.com/std_misc/threads.html
