// http://www.jsoftware.com/papers/play132.htm
//
// and porting this C++ solution
//     #include <vector>
//     #include <iostream>
//     using namespace std;
//     int main() {
//         const int n = 5;
//         const int dx[] = {0, 1, 0, -1}, dy[] = {1, 0, -1, 0};
//         int x = 0, y = -1, c = 0;
//         vector<vector<int>> m(n, vector<int>(n));
//         for (int i = 0, im = 0; i < n + n - 1; ++i, im = i % 4)
//         	for (int j = 0, jlen = (n + n - i) / 2; j < jlen; ++j)
//         		m[x += dx[im]][y += dy[im]] = ++c;
//         for (auto & r : m) {
//         	for (auto & v : r)
//         		cout << v << ' ';
//         	cout << endl;
//         }
//
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [1, 0, -1, 0];
    let mut x: i32 = 0;
    let mut y: i32 = -1;
    let mut n: i32 = 1;
    let ilim: i32 = (size + size) as i32 - 1;
    for i in 0..ilim {
        let jlim: i32 = ((size + size) as i32 - i) / 2;
        let im: i32 = i % 4;
        for _j in 0..jlim {
            x += dx[im as usize];
            y += dy[im as usize];
            println!("v[{}][{}]", x, y);
            v[x as usize][y as usize] = n as u32;
            n += 1;
        }
    }
    return v;
}

// order of evaluation:
//
// v[0][0] >
// v[0][1] ->
// v[0][2] -->
// v[0][3] --->
// v[0][4] ---->
//
// v[1][4]     |
// v[2][4]     |
// v[3][4]     |
// v[4][4]     v
//
// v[4][3]    <-
// v[4][2]   <--
// v[4][1]  <---
// v[4][0] <----

// v[3][0] ^
// v[2][0] |
// v[1][0] |

// v[1][1]
// v[1][2]
// v[1][3]

// v[2][3]
// v[3][3]

// v[3][2]
// v[3][1]

// v[2][1]
// v[2][2]
//
