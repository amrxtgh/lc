impl Solution {
    pub fn find_degrees(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sums: Vec<i32> = Vec::new();
        for rows in &matrix {
            sums.push(rows.iter().sum());
        }
        sums
    }
}