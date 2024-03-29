//https://doc.rust-lang.org/rand/rand/index.html

use rand::distributions::{Distribution, Range};

fn main() {
   let between = Range::new(-1f64, 1.);
   let mut rng = rand::thread_rng();

   let total = 1_000_000;
   let mut in_circle = 0;

   for _ in 0..total {
       let a = between.sample(&mut rng);
       let b = between.sample(&mut rng);
       if a*a + b*b <= 1. {
           in_circle += 1;
       }
   }

   // prints something close to 3.14159...
   println!("{}", 4. * (in_circle as f64) / (total as f64));
}
