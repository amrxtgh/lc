use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        for (i, &nums) in nums.iter().enumerate() {
            let complement = target - nums;
            if let Some(&prev_index) = seen.get(&complement) {
                return vec![prev_index as i32, i as i32];
            }
            seen.insert(nums, i);
        }
        vec![]
    }
}