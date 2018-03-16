use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
// use std::sync::mpsc::channel;

#[derive(Debug)]
struct Transaction {
    amount: isize,
    timestamp: u64,
    txid: String,
}

#[derive(Debug)]
struct AccountMutex {
    account_number: String,
    transactions: Mutex<Vec<Transaction>>,
    acct_type: String,
}

impl AccountMutex {
    pub fn new(s: String) -> AccountMutex {
        AccountMutex{
            account_number: s,
            transactions: Mutex::new(vec![]),
            acct_type: "mutexAccount".to_owned(),
        }
    }
}

#[derive(Debug)]
struct AccountChannel {
    account_number: String,
    transactions: Vec<Transaction>,
    acct_type: String,
}

// Using Threads to Run Code Simultaneously
// https://doc.rust-lang.org/book/second-edition/ch16-01-threads.html
// https://doc.rust-lang.org/std/thread/
// https://doc.rust-lang.org/std/thread/fn.spawn.html
//
// A new thread can be spawned using the thread::spawn function:
//
// use std::thread;
//
// thread::spawn(move || {
//     // some work here
// });
//
// In this example, the spawned thread is "detached" from the current
// thread. This means that it can outlive its parent (the thread that
// spawned it), unless this parent is the main thread.
//
// The parent thread can also wait on the completion of the child
// thread; a call to spawn produces a JoinHandle, which provides a join
// method for waiting:
//
// use std::thread;
//
// let child = thread::spawn(move || {
//     // some work here
// });
// // some work here
// let res = child.join();
//
// The join method returns a thread::Result containing Ok of the final
// value produced by the child thread, or Err of the value given to a
// call to panic! if the child panicked.

// Struct std::time::Duration
// https://doc.rust-lang.org/std/time/struct.Duration.html

fn mutex() {
    let my_savings = Arc::new(AccountMutex::new("0001".to_owned()));
    let feed_account = my_savings.clone();
    let mobile_account = my_savings.clone();

    // [...] we said that "If we want to force the closure to take
    // ownership of the values it uses in the environment, we can use
    // the =move= keyword before the parameter list. This technique is
    // mostly useful when passing a closure to a new thread to move the
    // data so it's owned by the new thread."

    let file_feed = thread::spawn(move || {
        let mut tx_guard = feed_account.transactions.lock().unwrap();

        tx_guard.push(Transaction {
            amount: 500,
            timestamp: 12,
            txid: "tx-001".to_owned(),
        });
        thread::sleep(Duration::from_millis(1));
        tx_guard.push(Transaction {
            amount: 750,
            timestamp: 4,
            txid: "tx-002".to_owned(),
        })
    });

    let mobile_feed = thread::spawn(move || {
        mobile_account
            .transactions
            .lock()
            .unwrap()
            .push(Transaction {
                amount: 50,
                timestamp: 7,
                txid: "tx-003".to_owned(),
            });
    });

    file_feed.join().unwrap();
    mobile_feed.join().unwrap();
}

fn channel() {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    let file_feed2 = thread::spawn(move || {
        tx.send(Transaction {
            amount: 500,
            timestamp: 12,
            txid: "ch-tx-001".to_owned(),
        }).unwrap();
        tx.send(Transaction {
            amount: 750,
            timestamp: 4,
            txid: "ch-tx-002".to_owned(),
        }).unwrap();
    });

    let mobile_feed2 = thread::spawn(move || {
        tx2.send(Transaction {
            amount: 50,
            timestamp: 7,
            txid: "ch-tx-003".to_owned(),
        }).unwrap();
    });

    file_feed2.join().unwrap();
    mobile_feed2.join().unwrap();

    let mut tl_savings = AccountChannel {
        acct_type: "Savings".to_owned(),
        account_number: "0001".to_owned(),
        transactions: Vec::new(),
    };

    for transaction in rx {
        tl_savings.transactions.push(transaction);
    }
}

fn main() {
    println!("running some code")
}
