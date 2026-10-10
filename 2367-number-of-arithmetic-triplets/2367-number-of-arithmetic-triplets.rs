use std::collections::HashSet;
impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut count = 0;
        let num_set : HashSet<i32> = nums.into_iter().collect();
        for &num in &num_set {
            if num_set.contains(&(num-diff)) && num_set.contains(&(num- 2 * diff)) {
                count += 1;
            } 
        }
        count
    }
}