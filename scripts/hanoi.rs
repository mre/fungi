// We need to move N plates from Tower 1 to Tower 3, but let’s start from the
// beginning.
//
// One plate
// - 1. move Plate 1 from Tower 1 to Tower 3
//
// Two plates
// - 1. Move Plate 1 from Tower 1 to Tower 2
// - 2. Move Plate 2 from Tower 1 to Tower 3
// - 3. Move Plate 1 from Tower 2 to Tower 3
//
// Three plates
// - 1. We know we can move the top two plates around from one Tower to
//      another (as shown earlier), so let's assume we have moved Plate 1 and
//      Plate 2 to Tower 2 (buffer).
//      - 1. Move Plate 1 from Tower 1 to Tower 3
//      - 2. Move Plate 2 from Tower 1 to Tower 2
//      - 3. Move Plate 1 from Tower 3 to Tower 2
// - 2. Move Plate 3 to Tower 3
// - 3. Again we know we can move the top two plates around, so let's move
//      them from Tower 2 to Tower 3
//      - 1. Move Plate 1 from Tower 2 to Tower 1
//      - 2. Move Plate 2 from Tower 2 to Tower 3
//      - 3. Move Plate 1 from Tower 1 to Tower 3
//
// This approach leads to a natural recursive algorithm.

use std::error::Error;
use std::fmt;

#[allow(dead_code)]
fn move_plates(n: usize, source: &mut Tower, destin: &mut Tower, buffer: &mut Tower) -> () {
    if n == 1 {
        // move source to destination directly
        source.move_top_to(destin);
    }
    if n == 2 {
        // move the 1st and 2nd plate using a buffer
        source.move_top_to(buffer);
        source.move_top_to(destin);
        buffer.move_top_to(destin);
    }
    if n == 3 {
        // move the 1st and 2nd plate using a buffer (from t1 to t2)
        // move from source to buffer.
        source.move_top_to(destin);
        source.move_top_to(buffer);
        destin.move_top_to(buffer);
        
        // move the 3rd plate to destination (from t1 to t3)
        // move from source to destination.
        source.move_top_to(destin);
        
        // move the 1st and 2nd place using a buffer (from t2 to t3)
        // move from buffer to destination.
        buffer.move_top_to(source);
        buffer.move_top_to(destin);
        source.move_top_to(destin);
    }
    if n == 4 {
        // move disk 0 from 0 to 1
        // move disk 1 from 0 to 2
        // move disk 0 from 1 to 2
        
        // move disk 2 from 0 to 1
        
        // move disk 0 from 2 to 0
        // move disk 1 from 2 to 1
        // move disk 0 from 0 to 1
        
        // move disk 3 from 0 to 2
        
        // move disk 0 from 1 to 2
        // move disk 1 from 1 to 0
        // move disk 0 from 2 to 0
        
        // move disk 2 from 1 to 2
        
        // move disk 0 from 0 to 1
        // move disk 1 from 0 to 2
        // move disk 0 from 1 to 2
    }
    if n == 5 {
         // move disk 0 from 0 to 2 
         // move disk 1 from 0 to 1 
         // move disk 0 from 2 to 1 
        
         // move disk 2 from 0 to 2 
        
         // move disk 0 from 1 to 0 
         // move disk 1 from 1 to 2 
         // move disk 0 from 0 to 2 
        
         // move disk 3 from 0 to 1 
        
         // move disk 0 from 2 to 1 
         // move disk 1 from 2 to 0 
         // move disk 0 from 1 to 0 
        
         // move disk 2 from 2 to 1 
        
         // move disk 0 from 0 to 2 
         // move disk 1 from 0 to 1 
         // move disk 0 from 2 to 1 
        
         // move disk 4 from 0 to 2 
        
         // move disk 0 from 1 to 0 
         // move disk 1 from 1 to 2 
         // move disk 0 from 0 to 2 
        
         // move disk 2 from 1 to 0 
        
         // move disk 0 from 2 to 1 
         // move disk 1 from 2 to 0 
         // move disk 0 from 1 to 0 
        
         // move disk 3 from 1 to 2 
        
         // move disk 0 from 0 to 2 
         // move disk 1 from 0 to 1 
         // move disk 0 from 2 to 1 
        
         // move disk 2 from 0 to 2 
        
         // move disk 0 from 1 to 0 
         // move disk 1 from 1 to 2 
         // move disk 0 from 0 to 2 
    }
}

#[allow(dead_code)]
fn initialise(n: usize) -> Vec<Tower> {
    let mut towers: Vec<Tower> = Vec::with_capacity(n);
    for i in 0..3 {
        let t: Tower = Tower {
            disks: Vec::<usize>::new(),
            index: i,
        };
        towers.push(t);
    }
    for i in (0..n).rev() {
        towers[0].add(i).unwrap();
    }
    return towers;
}

fn main() -> () {
    let n: usize = 5;
    // https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
    //
    // &mut towers[0];
    // towers.get_mut(0).expect("a tower collapsed");
    let mut t0: Tower = Tower::new(0);
    let mut t1: Tower = Tower::new(1);
    let mut t2: Tower = Tower::new(2);
    // load the first tower
    for i in (0..n).rev() {
        t0.add(i).unwrap();
    }
    println!("{}", &t0);
    println!("{}", &t1);
    println!("{}", &t2);
    println!("--");

    // move 5 disks (n) from the first tower (t0) to the last tower (t2)
    // using the middle tower (t1) as buffer.
    t0.move_disks(n, &mut t2, &mut t1);

    // iterative
    // move_plates(3, &mut t0, &mut t2, &mut t1);
    
    println!("--");
    println!("{}", &t0);
    println!("{}", &t1);
    println!("{}", &t2);
}

#[derive(Debug)]
struct Tower {
    disks: Vec<usize>,
    index: usize,
}

impl fmt::Display for Tower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Tower {:02}: stack: {:?}", self.index, self.disks);
    }
}

impl Tower {
    fn new(index: usize) -> Tower {
        return Tower {
            disks: Vec::<usize>::new(),
            index,
        };
    }

    fn index(&self) -> usize {
        return self.index;
    }

    fn add(&mut self, d: usize) -> Result<(), Box<dyn Error>> {
        if self.disks.first().is_some() && self.disks.first().unwrap() <= &d {
            return Err(Box::<Error>::from(format!("Error placing disk {}", d)));
        }
        self.disks.push(d);
        return Ok(());
    }

    fn move_top_to(&mut self, t: &mut Tower) {
        match self.disks.pop() {
            Some(top) => {
                match t.add(top) {
                    Ok(_) => {
                        println!("move disk {} from {} to {}", top, self.index(), t.index());
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
                return;
            }
            None => (),
        };
    }

    fn move_disks(&mut self, n: usize, destination: &mut Tower, buffer: &mut Tower) {
        // what we need to recurse here is a case, where for any n > 1
        // we do 3 steps:
        //  1. (decrease n) move orig to buff
        //  2. (direct)     move orig to dest
        //  3. (decrease n) move buff to dest
        // alternating these steps we traverse the tree of moves.
        if n > 0 {
            self.move_disks(n - 1, buffer, destination);
            self.move_top_to(destination);
            buffer.move_disks(n - 1, destination, self);
        }
    }
}
