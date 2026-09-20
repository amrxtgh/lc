use std::collections::HashSet;
impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let min_val = *nums.iter().min().unwrap();
        let max_val = *nums.iter().max().unwrap();
        let set: HashSet<i32> = nums.into_iter().collect();

        ((min_val + 1)..max_val).filter(|x| !set.contains(x)).collect()
    }
}