pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

// Pascal's triangle
// https://en.wikipedia.org/wiki/Pascal's_triangle
// https://en.wikipedia.org/wiki/Pascal%27s_rule
// https://en.wikipedia.org/wiki/Pascal_matrix
//
// givent the fact that the element at the row n, position k is
// (n k) = n!/((n-k)!k!)
//
fn mm(from: u32, to: u32) -> u32 {
    (from..to).fold(1, |sum, n| sum * (n + 1))
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::<Vec<u32>>::with_capacity(row_count as usize);

        for i in 0..row_count {
            let mut r: Vec<u32> = Vec::new();
            for j in 0..(i + 1) {
                r.push(mm(j, i) / mm(1, i - j));
            }
            rows.push(r);
        }
        return Self { rows };
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        return self.rows.to_owned();
    }
}
