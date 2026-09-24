impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..k {
            let min_index = nums.iter().enumerate()
            .min_by_key(|&(_idx, &val)| val)
            .map(|(idx, val)| idx)
            .unwrap();
            nums[min_index] *= multiplier;
        }
        nums
    }
}