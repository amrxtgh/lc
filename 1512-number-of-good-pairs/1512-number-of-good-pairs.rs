use std::collections::HashMap;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();

        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }
        counts.values().map(|&n| n*(n-1)/2).sum()
    }
}