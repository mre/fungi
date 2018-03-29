use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

// https://doc.rust-lang.org/std/sync/atomic/
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
// https://github.com/rust-lang-nursery/log
// https://github.com/sebasmagri/env_logger/
// https://docs.rs/env_logger/*/env_logger/
#[macro_use]
extern crate log;
extern crate env_logger;

static GLOBAL_SPAWNED_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
static GLOBAL_DROPPED_THREAD_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

// https://doc.rust-lang.org/1.6.0/std/thread/struct.JoinHandle.html

// [...] we can take ownership of the value inside the Box<T> using
// self: Box<Self> [...]
trait FnBox {
    fn call_box(self: Box<Self>);
}

// [...] This involves defining a new trait that has a method call_box
// that uses `self: Box<Self>` in its signature, defining that trait for
// any type that implements FnOnce() [...]
impl<F: FnOnce()> FnBox for F {
    //  uses (*self)() to move the closure out of the Box<T> and call the closure
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// Job is going to be a type alias for a trait object that holds the
// type of closure that execute receives.
type Job = Box<FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

// Initially we had a "plain" thread::JoinHandle in the thread field of the
// Worker, but in the graceful shutdown we inccurred in this error:
//
// error[E0507]: cannot move out of borrowed content
//   --> src/lib.rs:65:13
//    |
// 65 |             worker.thread.join().unwrap();
//    |             ^^^^^^ cannot move out of borrowed content
//
// we only have a mutable borrow of each worker, we can’t call join:
// join takes ownership of its argument. In order to solve this, we need
// a way to move the thread out of the Worker instance that owns thread
// so that join can consume the thread.
// [...] if the Worker holds an Option<thread::JoinHandle<()>
// instead, we can call the take method on the Option to move the value
// out of the Some variant and leave a None variant in its place. In
// other words, a Worker that is running will have a Some variant in
// thread, and when we want to clean up a worker, we’ll replace Some
// with None so the worker doesn’t have a thread to run.

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI64.html#method.fetch_add
            let old_thread_count = GLOBAL_SPAWNED_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
            info!("live threads: {}", old_thread_count + 1);
            println!("live threads: {}", old_thread_count + 1);

            // here in a clojure
            let message = receiver
                .lock()
                .expect("cannot get the lock")
                .recv()
                .unwrap();

            info!("Worker {} got a job; executing.", id);

            match message {
                Message::NewJob(job) => {
                    // (*job)();
                    // error[E0161]: cannot move a value of type
                    // std::ops::FnOnce() + std::marker::Send: the size of
                    // std::ops::FnOnce() + std::marker::Send cannot be
                    // statically determined
                    //
                    // In order to call a FnOnce closure that is stored in a
                    // Box<T> (which is what our Job type alias is), the closure
                    // needs to be able to move itself out of the Box<T> since
                    // when we call the closure, it takes ownership of self.
                    //
                    // In general, moving a value out of a Box<T> isn’t allowed
                    // since Rust doesn’t know how big the value inside the
                    // Box<T> is going to be; [...] we used Box<T> precisely
                    // because we had something of an unknown size that we
                    // wanted to store in a Box<T> to get a value of a known
                    // size.
                    //
                    // we can write methods that use the syntax self: Box<Self>
                    // so that the method takes ownership of a Self value that
                    // is stored in a Box<T>.
                    //
                    // [...] there’s a trick that involves telling Rust
                    // explicitly that we’re in a case where we can take
                    // ownership of the value inside the Box<T> using self:
                    // Box<Self>, and once we have ownership of the closure, we
                    // can call it.
                    //
                    // defining a new trait that has a method call_box that uses
                    // self: Box<Self> in its signature, defining that trait for
                    // any type that implements FnOnce(), changing our type
                    // alias to use the new trait, and changing Worker to use
                    // the call_box method.

                    // [...] we use call_box instead of invoking the closure directly.
                    job.call_box();
                }
                Message::Terminate => {
                    info!("worker {} was told to terminate.", id);

                    // https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI64.html#method.fetch_sub
                    // https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
                    // let old_thread_count = GLOBAL_SPAWNED_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
                    //info!("live threads: {}", old_thread_count - 1);
                    let old_dropped_count =
                        GLOBAL_DROPPED_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
                    info!("dropped threads: {}", old_dropped_count + 1);
                    println!("dropped threads: {}", old_dropped_count + 1);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // https://doc.rust-lang.org/std/sync/mpsc/
        // https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html
        // https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
        let (sender, receiver) = mpsc::channel();
        // [...] we put the receiving end of the channel in an Arc and a
        // Mutex. For each new worker, we clone the Arc to bump the
        // reference count so the workers can share ownership of the
        // receiving end.
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
}

// [...] to implement the Drop trait for ThreadPool to call join on each of
// the threads in the pool so that the threads will finish the requests
// they’re working on. Then we’ll implement a way for the ThreadPool to
// tell the threads they should stop accepting new requests and shut
// down.
// When the pool is dropped, we should join on all of our threads to
// make sure they finish their work.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        info!("sending Terminate message to all workers");

        // Sending Message::Terminate to the workers before calling join
        // on each worker thread.
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        info!("shutting down all workers");

        for worker in &mut self.workers {
            info!("shutting down worker {}", worker.id);

            // [..] the take() method on Option takes the Some variant out and
            // leaves None in its place.
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T + Send + 'static,
//         T: Send + 'static
//
// F is the parameter we care about here; T is related to the return
// value and we’re not concerned with that. Given that spawn uses FnOnce
// as the trait bound on F, it’s probably what we want as well, since
// we’ll eventually be passing the argument we get in execute to
// spawn. We can be further confident that FnOnce is the trait that we
// want to use since the thread for running a request is only going to
// execute that request’s closure one time.
//
// F also has the trait bound Send and the lifetime bound 'static, which
// also make sense for our situation: we need Send to transfer the
// closure from one thread to another, and 'static because we don’t know
// how long the thread will execute. Let’s create an execute method on
// ThreadPool that will take a generic parameter F with these bounds:
//
// The FnOnce trait still needs the () after it since this FnOnce is
// representing a closure that takes no parameters and doesn’t return a
// value. Just like function definitions, the return type can be omitted
// from the signature, but even if we have no parameters, we still need
// the parentheses.
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }

    #[test]
    fn it_drops() {
        // outside the scope...

        // https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html
        // https://doc.rust-lang.org/std/sync/atomic/struct.AtomicI64.html#method.load
        assert_eq!(GLOBAL_SPAWNED_THREAD_COUNT.load(Ordering::SeqCst), 0);
        assert_eq!(GLOBAL_DROPPED_THREAD_COUNT.load(Ordering::SeqCst), 0);
        let l: usize = 3;
        // entering a new scope...
        {
            // inside the scope.
            assert_eq!(GLOBAL_SPAWNED_THREAD_COUNT.load(Ordering::SeqCst), 0);
            let tp: ThreadPool = ThreadPool::new(l);
            assert_eq!(l, tp.workers.len());

            // https://doc.rust-lang.org/std/time/struct.Duration.html
            use std::time::Duration;

            // TODO: bad, bad timed test here.
            // https://doc.rust-lang.org/std/thread/fn.sleep.html
            thread::sleep(Duration::from_millis(1000));
            
            assert_eq!(GLOBAL_SPAWNED_THREAD_COUNT.load(Ordering::SeqCst), l);
        }
        // out of the scope, where the ThreadPool gets dropped.
        assert_eq!(GLOBAL_SPAWNED_THREAD_COUNT.load(Ordering::SeqCst), l);
        assert_eq!(GLOBAL_DROPPED_THREAD_COUNT.load(Ordering::SeqCst), l);
        // consider this as an invariant, to be true at the end.
        assert_eq!(
            GLOBAL_DROPPED_THREAD_COUNT.load(Ordering::SeqCst)
                - GLOBAL_DROPPED_THREAD_COUNT.load(Ordering::SeqCst),
            0
        );
    }
}
