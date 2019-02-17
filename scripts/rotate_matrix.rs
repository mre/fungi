fn main() -> () {
    let mut m: [[usize; 6]; 6] = [[0; 6]; 6];
    let mut n: usize = 0;
    for i in 0..6 {
        for j in 0..6 {
            m[i][j] = n;
            n += 1;
        }
    }
    printm(&m);
    println!("---");
    rotate(&mut m, 6);
    printm(&m);
}

fn printm(m: &[[usize; 6]; 6]) -> () {
    for r in 0..6 {
        for c in 0..6 {
            print!("[({:?},{:?}),{:02?}]", r, c, &m[r][c]);
        }
        println!("");
    }
    println!("---");
}

fn rotate(matrix: &mut [[usize; 6]; 6], n: usize) {
    // we rotate this matrix by concentric frames
    for layer in 0..n / 2 {
        // first is the top edge of the frame
        let mut first: usize = layer;
        // last is the lower edge of the frame
        let mut last: usize = n - 1 - layer;

        println!("first {:?}", first);
        println!("last {:?}", last);

        // the distance of the frames is the number of steps
        // to take to do a full rotation
        for i in first..last {
            // i is driving the rotation as reference
            let mut offset: usize = i - first;
            println!("offset {:?}", offset);
            let mut top = matrix[first][i]; // save top

            // offset 0 is moving the corners
            // [0] -> [1]
            //  ^      |
            //  |      v
            // [3] <- [2]
            //
            // offset 1
            // [ ][0] -> [ ]
            //  ^        [1]
            //  |         |
            //  |         |
            // [3]        v
            // [ ] <- [2][ ]

            // from the left moving to top
            println!(
                "swapping ({},{}) with ({},{})",
                first,
                i,
                last - offset,
                first
            );
            matrix[first][i] = matrix[last - offset][first];

            // bottom -> left
            println!(
                "swapping ({},{}) with ({},{})",
                last - offset,
                first,
                last,
                last - offset
            );
            matrix[last - offset][first] = matrix[last][last - offset];

            // right -> bottom
            println!(
                "swapping ({},{}) with ({},{})",
                last,
                last - offset,
                i,
                last
            );
            matrix[last][last - offset] = matrix[i][last];

            // top -> right
            println!("swapping ({},{}) with ({},{})", i, last, first, i);
            matrix[i][last] = top; // right <- saved top
            println!("---");
        }
    }
}
