impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut operations = 0;
        for i in 0..n {
            if nums[i] < k {
                operations += 1;
            }
        }
        operations
    }
}