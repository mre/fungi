use std::error::Error;
use std::fmt;

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

    // public void print() {
    // System.out.println(“Contents of Tower “ + index()); for (int i = disks.size() - 1; i >= 0; i--) {
    // System.out.println(“ “ + disks.get(i)); }
    // }

    fn move_disks(&mut self, n: usize, destination: &mut Tower, buffer: &mut Tower) {
        if n > 0 {
            self.move_disks(n - 1, buffer, destination);
            self.move_top_to(destination);
            buffer.move_disks(n - 1, destination, self);
        }
    }
}
