impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let total_sum: i32 = nums.iter().sum();
        let mut left_sum: i32 = 0;
        let mut ans = Vec::with_capacity(nums.len());

        for &nums in nums.iter() {
            let right_sum = total_sum - left_sum - nums;
            let diff = (left_sum - right_sum).abs();
            ans.push(diff);
            left_sum += nums; 
        } 
        ans
    }
}