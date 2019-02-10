// https://en.wikipedia.org/wiki/Water_pouring_puzzle
// https://demonstrations.wolfram.com/WaterPouringProblem/
// https://damiannmm.github.io/blog/two-buckets-problem/
// https://en.wikipedia.org/wiki/Linear_combination
// https://en.wikipedia.org/wiki/Bézout%27s_identity
//
// Given two buckets, each sized m and n, is it possible to have an x amount
// of water in any of the bucket, given "{m , n , x} ∈ N" ?
// It is possible if "x ≤ max (m , n)" and "gcd (m , n) ∣ x" .
// "a|b" read "a divides b" or "a is a factor of b"
// the given problem could be reduced to
// does a pair of integer p and q that satisfies "p m + q n = x" exists?
//
// By the bezout’s identity–with "g = gcd (m , n)" , there exists
// integers a and b such that "a m + b n = g" . If "g ∣ x" , or in other
// words exists y where "y ⋅ g = x" , then there's pair of p and q such
// that "p m + q n = x", at "(p , q) = (y ⋅ a , y ⋅ b)" .  Alas, a m + b
// n = g ⇔ y (a m + b n) = y g ⇔ y a ⋅ m + y b ⋅ n = x ⇔ p m + q n = x
//
// Which in other word, it is possible, one of the given bucket could be
// filled by x .
//
// The water jug problem can be solved using the extended-eucildean
// algorithm. Extended-euclidean algorithm finds solution for
// diophantine equations. How does finding solution of diophantine
// equation solves the water jug problem? Let me demonstrate:
//
// Imagine you have a jug of 5 liters ( A ) and 3 liters ( B ). You want
// to make 1 liter of water with this 2 container. The the equation will
// be, 5x + 3y = 1 ( Ax + By = 1 ). If we can find a solution this
// equation then our problem is solved. Apply extended_euclidean
// algorithm on it and you will find that the result is x = 2 and y =
// -3.
//
// If we put value of x and y in the equation then we get 5 * 2 + 3 * (
// -3 ) = 10 - 9 = 1. The equation is indeed solved. But what does x = 2
// and y = -3 mean? It means, we need to fill our A bottle 2 times and
// empty our B bottle 3 times.
//
// This how it will go:
//
// A                B
// 5 (fill Once )
// 2                3 ( Transfer 3 liter )
// 2                0 ( Empty Once )
// 0                2 ( Transfer 2 liter )
// 5 (fill Twice )  2
// 4                3 ( Transfer 1 liter )
// 4                0 ( Empty Twice )
// 1                3 ( Transfer 3 liter )
// 1                0 ( Empty Thrice )
//

use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

fn gcd(n: u8, m: u8) -> u8 {
    // variable names based off Euclidean divison equation: a = b · q + r
    let (mut a, mut b) = if n > m { (n, m) } else { (m, n) };

    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    return a;
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    // assert that the goal is possible
    assert!(goal % gcd(capacity_1, capacity_2) == 0);
    let mut heap: BinaryHeap<(u8, u8, u8)> = BinaryHeap::new();
    // let mut heap: Vec<(u8, u8, u8)> = Vec::new();
    heap.push((0, 0, 0));
    let mut seen = HashSet::new();
    match start_bucket {
        &Bucket::One => seen.insert((0, capacity_2)),
        &Bucket::Two => seen.insert((capacity_1, 0)),
    };
    while let Some((m, b1, b2)) = heap.pop() {
        if seen.contains(&(b1, b2)) {
            continue;
        }
        // mark this status as "seen"
        seen.insert((b1, b2));
        if b1 == goal {
            return BucketStats {
                moves: m,
                goal_bucket: Bucket::One,
                other_bucket: b2,
            };
        } else if b2 == goal {
            return BucketStats {
                moves: m,
                goal_bucket: Bucket::Two,
                other_bucket: b1,
            };
        }
        let n: u8 = m.saturating_add(1);
        // trasfer content from b1 to b2
        let (next_b1, next_b2) = if b1.saturating_add(b2) > capacity_2 {
            // fill b2 and leave in b1 the rest
            (b1.saturating_add(b2) - capacity_2, capacity_2)
        } else {
            // empty b1 in b2
            (0, b2 + b1)
        };
        heap.push((n, next_b1, next_b2));
        // trasfer content from b2 to b1
        let (next_b2, next_b1) = if b2.saturating_add(b1) > capacity_1 {
            // fill b1 and leave in b2 the rest
            (b2.saturating_add(b1) - capacity_1, capacity_1)
        } else {
            // empty b1 in b2
            (0, b1 + b2)
        };
        heap.push((n, next_b1, next_b2));
        // fill the buckets
        heap.push((n, capacity_1, b2));
        heap.push((n, b1, capacity_2));
        // empty the buckets
        heap.push((n, 0, b2));
        heap.push((n, b1, 0));
    }
    BucketStats {
        moves: 0,
        goal_bucket: Bucket::One,
        other_bucket: 0,
    }
}
