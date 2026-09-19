impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Self::backtrack(&nums, 0, 0)
    }
    fn backtrack(nums: &Vec<i32>, index: usize, current_xor: i32) -> i32 {
        if index == nums.len() {
            return current_xor;
        }
        let skip = Self::backtrack(nums, index+1,current_xor);
        let take = Self::backtrack(nums, index+1, current_xor ^ nums[index]);
        skip + take
    }
}