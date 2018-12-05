pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();
    for (r, row) in input.iter().enumerate() {
        for (c, &num) in row.iter().enumerate() {
            let rs = row.iter().map(|&d| d);
            let cs = input.iter().map(|row| row[c]);
            if num == cs.min().unwrap() && num == rs.max().unwrap() {
                saddle_points.push((r, c));
            }
        }
    }
    saddle_points
}
