extern crate rand;
extern crate futures;

use futures::Future;
use futures::future::FutureResult;
use futures::future::Join;
use futures::future::ok;
use rand::Rng;

// https://docs.rs/crate/futures/0.1.19
// https://docs.rs/futures/0.2.0-beta/futures/

// Box<Future<Item=u64, Error=()>>
fn fut_num() -> Join<FutureResult<u32, ()>, FutureResult<u32, ()>>  {
    // a RNG that is local to the current thread of execution and seeded by the
    // operating system
    let a = ok::<_, ()>(rand::thread_rng().gen_range(0, 10));
    let b = ok::<_, ()>(rand::thread_rng().gen_range(0, 10));    
    // Here I specify the type of the error as (); otherwise the compiler can't
    // infer it.
    a.join(b)
}

fn main() {
  let n = fut_num();
  let r = n.map(|(x, y)| x + y).wait();
  println!("Waited for two futures: {:?}", r.unwrap())
}

// extern crate futures;
// extern crate tokio_core;
// fn main() {
//     use tokio_core::reactor::Core;
//     use futures::future::lazy;
//     let mut core = Core::new().unwrap();
//     let handle = core.handle();
//     let future = lazy(|| {
//         handle.spawn(lazy(|| {
//             Ok(()) // Ok(()) implements FromResult
//         }));
//         Ok(2)
//     });
//     let expected: Result<_, ()> = Ok(2usize);
//     assert_eq!(core.run(future), expected);
// }
