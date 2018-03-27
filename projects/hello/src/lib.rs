use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
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
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
}

// pub fn spawn<F, T>(f: F) -> JoinHandle<T>
//     where
//         F: FnOnce() -> T + Send + 'static,
//         T: Send + 'static

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

    }
}
