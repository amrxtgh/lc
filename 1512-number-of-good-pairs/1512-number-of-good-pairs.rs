use std::collections::HashMap;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count = [0; 101];
        let mut result = 0;
        for num in nums {
            let index = num as usize;
            result += count[index];
            count[index] += 1;
        }
        result
    }
}