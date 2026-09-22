impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();

        // Creates a 2D Vector of size (n - 2) x (n - 2) filled with zeros
        let mut maxLocal = vec![vec![0; n - 2]; n - 2];


        for i in 0..(n - 2) {
            for j in 0..(n - 2) {
                maxLocal[i][j] = [grid[i + 1][j + 1], grid[i][j + 1], grid[i + 2][j + 1], grid[i + 1][j], grid[i + 1][j + 2], grid[i][j], grid[i + 2][j + 2], grid[i][j + 2], grid[i + 2][j]].into_iter().max().unwrap();
            }
        }

        maxLocal
    }
}