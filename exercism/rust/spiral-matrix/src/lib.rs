
// http://www.jsoftware.com/papers/play132.htm
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];
    let mut x: i32 = 0;
    let mut y: i32 = -1;
    let mut n: i32 = 0;
    let ilim: i32 = (size + size) as i32 -1;
    let mut im: i32 = 0;
    for i in 0..ilim {
        let jlim: i32 = ilim /2;
        for _j in 0..jlim {
            x += dx[im as usize];
            y += dy[im as usize];
            v[x as usize][y as usize] = n as u32;
            n += 1;
        }
        im = i % 4;
    }
    return v;
}

