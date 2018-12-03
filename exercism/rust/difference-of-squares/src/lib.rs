// 1 + 2 + 3 + 4 + ⋯
// From Wikipedia, the free encyclopedia
// https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_⋯
pub fn square_of_sum(n: usize) -> usize {
    // (1..n+1).sum::<usize>().pow(2)
    n * n * (n + 1) * (n + 1) / 4
}

// According to https://en.wikipedia.org/wiki/Square_pyramidal_number
pub fn sum_of_squares(n: usize) -> usize {
    // (1..n+1).map(|x| x.pow(2)).sum::<usize>()
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
