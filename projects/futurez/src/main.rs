extern crate futures;
extern crate rand;

use futures::Future;
use futures::future::Either;
use futures::future::FutureResult;
use futures::future::Join;
use futures::future::ok;
use rand::Rng;

// https://docs.rs/crate/futures/0.1.19
// https://docs.rs/futures/0.2.0-beta/futures/

// Box<Future<Item=u64, Error=()>>
fn fut_num() -> Join<FutureResult<u32, ()>, FutureResult<u32, ()>> {
    // a RNG that is local to the current thread of execution and seeded by the
    // operating system
    let a = ok::<_, ()>(rand::thread_rng().gen_range(0, 10));
    let b = ok::<_, ()>(rand::thread_rng().gen_range(0, 10));
    // Here I specify the type of the error as (); otherwise the compiler can't
    // infer it.
    a.join(b)
}

// https://docs.rs/futures/0.1.19/futures/future/enum.Either.html#variant.A
// fn under_n_or_false<T: Future, U: Future>(n: u32) -> Either<T, U> {
// https://www.ncameron.org/blog/abstract-return-types-aka-%60impl-trait%60/
// fn under_n_or_false<T, U>(n: u32) -> Either<T, U> where T: Future, U: Future,
fn under_n_or_false(n: u32) -> Either<FutureResult<u32, ()>, FutureResult<bool, ()>>
{
    let r = rand::thread_rng().gen_range(0, 10);
    if r < n {
        let t: FutureResult<_, _> = ok::<_, ()>(r);
        Either::A(t)
    } else {
        let u: FutureResult<_,_> = ok::<_,_>(false);
        Either::B(u)
    }
}

// https://docs.rs/futures/0.1.19/futures/future/trait.Future.html
// https://docs.rs/futures/0.1.19/futures/future/struct.FutureResult.html
fn main() {
    let n = fut_num();
    let r = n.map(|(x, y)| x + y).wait();
    println!("Waited for two futures: {:?}", r.unwrap());
    match under_n_or_false(3) {
        Either::A(r) => println!("waiting was good: {:?}", r.wait().unwrap()),
        Either::B(r) => println!("wasn't worth waiting: {:?}", r.wait().unwrap()),
    }
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
