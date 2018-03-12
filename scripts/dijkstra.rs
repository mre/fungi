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

// StateDisplay is a "marker value". It's a struct wrapping the type we want to
// print is a single value tuple.
// We use this `StateDisplay` to carry around our Option type.
struct StateDisplay<'a>(Option<&'a State>);

// CustomStateDisplay is a trait that introduce the `custom_display` function to
// its implementors. In this signature, `custom_display` takes just the `self`
// that is actually implementing this trait, and returns a `StateDisplay`. So,
// `custom_display` is a function that will return a StateDisplay with the same
// lifetime (some sort of a "constuctor").
trait CustomStateDisplay {
    fn custom_display<'a>(&'a self) -> StateDisplay<'a>;
}

// For the type that we want to print (using the Display trait), the
// implementation of the trait CustomStateDisplay is obvious (since the
// StateDisplay is a single value tuple struct around it).
// With this, our Option type will now be an implementor of the
// CustomStateDisplay.
impl<'b> CustomStateDisplay for Option<&'b State> {
    fn custom_display<'a>(&'a self) -> StateDisplay<'a> {
        StateDisplay(*self)
    }
}

// Since StateDisplay is "the thing that we can print", we can now properly
// implement the "Trait std::fmt::Display" from its definition:
//   - https://doc.rust-lang.org/std/fmt/
//   - https://doc.rust-lang.org/std/fmt/trait.Display.html
//
// pub trait Display { fn fmt(&self, f: &mut Formatter) -> Result<(), Error>; }
//
// Doing this we will have the elements needed for printing a custom Option
// type: the wrapper for the type, the function to wrap the Option type and the
// implementation of Display for the wrapper.
// We only need to call the "wrapping function" (custom_display) from our
// Option type and use the result within a "Displayable" contenxt, like print!.
impl<'a> fmt::Display for StateDisplay<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
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
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Returns a Vector of usize of length l.
//
// * Primitive Type usize
// The pointer-sized unsigned integer type.
// The size of this primitive is how many bytes it takes to reference
// any location in memory. For example, on a 32 bit target, this is 4
// bytes and on a 64 bit target, this is 8 bytes.
// https://doc.rust-lang.org/std/primitive.usize.html
fn distance_vector(l: usize) -> Vec<usize> {
    return (0..l).map(|_| usize::MAX).collect();
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
    let mut dist: Vec<_> = distance_vector(adj_list.len());

    // Module std::collections::binary_heap
    // https://doc.rust-lang.org/std/collections/binary_heap/index.html
    //
    // Struct std::collections::binary_heap::BinaryHeap
    // A priority queue implemented with a binary heap.
    // This will be a min-heap.
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost.
    // We can use dist as an hashmap.
    dist[start] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    println!(
        " - the start node is: {} (top of the heap), the graph has {} nodes",
        heap.peek().custom_display(),
        dist.len(),
    );

    // Examine the frontier with lower cost nodes first (min-heap)
    //
    // we cannot use a while-let (without matching on Some here because of E0165
    // while let top = heap.pop() {
    //
    //
    // rustc --explain E0165
    // A while-let pattern attempts to match the pattern, and enters the
    // body if the match was successful. If the match is irrefutable
    // (when it cannot fail to match), use a regular `let`-binding
    // inside a `loop` instead.
    //
    // while let Some(State { cost, position }) = heap.pop() {
    loop {
        // https://doc.rust-lang.org/std/option/enum.Option.html
        //
        // as_ref
        //   pub fn as_ref(&self) -> Option<&T>
        // Converts from Option<T> to Option<&T>.
        //
        // expect
        //   pub fn expect(self, msg: &str) -> T
        // Unwraps an option, yielding the content of a Some.

        let top: Option<State> = heap.pop();
        println!(" - popped node: {}", top.as_ref().custom_display());
        if top.is_none() {
            println!("the heap is empty");
            break;
        }

        let State { cost, position } = top.expect("the world is ending");

        // Alternatively we could have continued to find all shortest paths.
        // The node popped from the heap is the destination to reach, the
        // cost is included in the State.
        if position == goal {
            println!(" [goal reached] returning: cost is {}", cost);
            return Some(cost);
        }

        // Important as we may have already found a better way.
        // The cost in the State that we popped from the heap is greater than
        // the cost in the corresponding vector of distances. This state can be
        // discarded (continue).
        if cost > dist[position] {
            println!(" - [skip] state cost is greater than the one found");
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        println!(
            "   from position {} we have {} edges",
            position,
            &adj_list[position].len()
        );
        for edge in &adj_list[position] {
            println!(
                "   - considering edge: cost {} to go to: {}",
                edge.cost, edge.node
            );
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            println!(
                "   - next possible state, cost: {}, node: {}",
                next.cost, edge.node
            );

            // If so, add it to the frontier and continue
            println!(
                "     checking if is worth reaching \
                 the node {} with the new cost (was {})",
                next.position, cost
            );
            if next.cost < dist[next.position] {
                println!(
                    "     better cost found ({}) to go to {}: [pushed to heap]",
                    next.cost, next.position
                );
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            } else {
                println!("     nope, ignoring this new state [discard]");
            }
        }
    }

    // Goal not reachable
    None
}

fn from_to(f: usize, t: usize) -> () {
    let s: String = format!(
        "\n\n=== Dijkstra shortest path, from {from} to {to}\n",
        from = f,
        to = t
    );
    print!("{}", s)
}

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
fn initial_graph() -> Vec<Vec<Edge>> {
    vec![
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
    ]
}

// rustc scripts/dijkstra.rs --out-dir ./target && ./target/dijkstra
fn main() {
    let graph = initial_graph();
    from_to(0, 1);
    assert_eq!(shortest_path(&graph, 0, 1), Some(1));
    from_to(0, 3);
    assert_eq!(shortest_path(&graph, 0, 3), Some(3));
    from_to(3, 0);
    assert_eq!(shortest_path(&graph, 3, 0), Some(7));
    from_to(0, 4);
    assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    from_to(4, 0);
    assert_eq!(shortest_path(&graph, 4, 0), None);
}
