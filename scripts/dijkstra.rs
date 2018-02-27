// https://doc.rust-lang.org/std/collections/binary_heap/
// This is a larger example that implements Dijkstra's algorithm to solve the
// shortest path problem on a directed graph. It shows how to use BinaryHeap
// with custom types.
//
// - https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// - https://en.wikipedia.org/wiki/Shortest_path_problem

use std::fmt;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// StateDisplay is a "marker value".
struct StateDisplay<'a>(Option<Box<&'a State>>);

// CustomStateDisplay is a trait that introduce the `display` function to its
// implementer. In this signature, `display` takes just the `self` that is
// actually implementing this trait, and returns a `StateDisplay`.
trait CustomStateDisplay {
    fn display<'a>(&'a self) -> StateDisplay<'a>;
}

// For the type that we want to Display, the implementation of the trait
// CustomStateDisplay is obvious (since the StateDisplay is a single value
// tuple struct around it).
impl<'b> CustomStateDisplay for Option<Box<&'b State>> {
    fn display<'a>(&'a self) -> StateDisplay<'a> {
        StateDisplay(self)
    }
}

impl<'a> fmt::Display for StateDisplay<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            Some(ref sd) => write!(formatter, "pos: {}, cost: {}", sd.position, sd.cost),
            None => write!(formatter, "No struct"),
        }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
//
// It uses Enum std::cmp::Ordering#then_with
// https://doc.rust-lang.org/std/cmp/enum.Ordering.html#method.then_with
//
// pub fn then_with<F>(self, f: F) -> Ordering
// where F: FnOnce() -> Ordering,
//
//     Chains the ordering with the given function.
//     Returns self when it's not Equal. Otherwise calls f and returns the result.
//     Examples
//
// use std::cmp::Ordering;
// let result = Ordering::Equal.then_with(|| Ordering::Less);
// assert_eq!(result, Ordering::Less);
// let result = Ordering::Less.then_with(|| Ordering::Equal);
// assert_eq!(result, Ordering::Less);
// let result = Ordering::Less.then_with(|| Ordering::Greater);
// assert_eq!(result, Ordering::Less);
// let result = Ordering::Equal.then_with(|| Ordering::Equal);
// assert_eq!(result, Ordering::Equal);
// let x: (i64, i64, i64) = (1, 2, 7);
// let y: (i64, i64, i64)  = (1, 5, 3);
// let result = x.0.cmp(&y.0).then_with(|| x.1.cmp(&y.1)).then_with(|| x.2.cmp(&y.2));
// assert_eq!(result, Ordering::Less);
//
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance to
// each node. This implementation isn't memory-efficient as it may leave
// duplicate nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // Everything is initialised at the MAX possible value (like infinity).
    // dist[node] = current shortest distance from `start` to `node`
    // Since a node is a usize, it can be used to index the Vec and get the same
    // behaviour of a HashMap.
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    // Module std::collections::binary_heap
    // https://doc.rust-lang.org/std/collections/binary_heap/index.html
    //
    // Struct std::collections::binary_heap::BinaryHeap
    // A priority queue implemented with a binary heap.
    // This will be a min-heap.
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    println!("start node: {} with dist: {}", start, dist[start]);

    heap.push(State {
        cost: 0,
        position: start,
    });
    println!("top of the heap is {}", heap.peek().display());

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

// rustc scripts/dijkstra.rs --out-dir ./target && ./target/dijkstra
fn main() {
    // This is the directed graph we're going to use.
    // The node numbers correspond to the different states,
    // and the edge weights symbolize the cost of moving
    // from one node to another.
    // Note that the edges are one-way.
    //
    //                  7
    //          +-----------------+
    //          |                 |
    //          v   1        2    |  2
    //          0 -----> 1 -----> 3 ---> 4
    //          |        ^        ^      ^
    //          |        | 1      |      |
    //          |        |        | 3    | 1
    //          +------> 2 -------+      |
    //           10      |               |
    //                   +---------------+
    //
    // The graph is represented as an adjacency list where each index,
    // corresponding to a node value, has a list of outgoing edges.
    // Chosen for its efficiency.
    let graph = vec![
        // Node 0
        vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
        // Node 1
        vec![Edge { node: 3, cost: 2 }],
        // Node 2
        vec![
            Edge { node: 1, cost: 1 },
            Edge { node: 3, cost: 3 },
            Edge { node: 4, cost: 1 },
        ],
        // Node 3
        vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
        // Node 4
        vec![],
    ];

    assert_eq!(shortest_path(&graph, 0, 1), Some(1));
    assert_eq!(shortest_path(&graph, 0, 3), Some(3));
    assert_eq!(shortest_path(&graph, 3, 0), Some(7));
    assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    assert_eq!(shortest_path(&graph, 4, 0), None);
}
