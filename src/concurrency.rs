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

// Channels
fn three() {
    // mpsc stands for multiple producer, single consumer.
    // https://doc.rust-lang.org/stable/std/sync/mpsc/index.html
    // https://doc.rust-lang.org/stable/std/sync/mpsc/struct.Sender.html
    use std::sync::mpsc;

    // The mpsc::channel function returns a tuple, the first element of which is
    // the sending end and the second element the receiving end. The
    // abbreviations tx and rx are traditionally used in many fields for
    // transmitter and receiver respectively.
    let (tx, rx) = mpsc::channel();
    // Moving tx to a spawned thread and sending "hi"
    thread::spawn(move || {
        let val = String::from("hi");
        // unwrap is a shortcut method that if the Result value is the Ok
        // variant, it will return the value inside the Ok. If the Result is the
        // Err variant, unwrap will call the panic! macro.
        // expect, which is similar to unwrap, lets us also choose the panic!
        // error message. Using expect instead of unwrap and providing good
        // error messages can convey your intent and make tracking down the
        // source of a panic easier.
        // tx.send(val).unwrap() {
        //
        // https://doc.rust-lang.org/stable/std/sync/mpsc/struct.Sender.html#method.send
        // A successful send occurs when it is determined that the other end of
        // the channel has not hung up already. An unsuccessful send would be
        // one where the corresponding receiver has already been deallocated.
        // Note that a return value of Err means that the data will never be
        // received, but a return value of Ok does not mean that the data will
        // be received. It is possible for the corresponding receiver to hang up
        // immediately after this function returns Ok.
        // This method will never block the current thread.
        // Struct std::sync::mpsc::SendError
        // An error returned from the Sender::send
        // https://doc.rust-lang.org/stable/std/sync/mpsc/struct.SendError.html
        match tx.send(val) {
            Ok(_) => (),
            Err(error) => panic!("There was a problem sending a message: {:?}", error),
        };

        // println!("val is {}", val);
        // error[E0382]: use of moved value: `val`
        // The send function takes ownership of its parameter, and when the
        // value is moved the receiver takes ownership of it.
    });
    // We're again using thread::spawn to create a new thread, and then use move
    // to move tx into the closure so the spawned thread owns tx. The spawned
    // thread needs to own the transmitting end of the channel in order to be
    // able to send messages through the channel.
    //
    //  The send method returns a Result<T, E> type, so that if the receiving
    //  end has already been dropped and there's nowhere to send a value, the
    //  send operation will error. In this example, we're simply calling unwrap
    //  to panic in case of error.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    // The receiving end of a channel has two useful methods: recv and try_recv.
    // We're using recv, short for receive, which will block the main thread's
    // execution and wait until a value is sent down the channel. Once a value
    // is sent, recv will return it in a Result<T, E>. When the sending end of
    // the channel closes, recv will return an error to signal that no more
    // values will be coming.
    // The try_recv method doesn't block, but will instead return a Result<T, E>
    // immediately: an Ok value holding a message if one is available, and an
    // Err value if there aren't any messages this time.
}

// Multiple messages
fn four() {
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// create multiple producers by cloning the transmitter
fn five() {
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn sample() {
    one();
    two();
    three();
    four();
    five();
}
