impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = Vec::with_capacity(n);
        let mut sum = 0;
        for i in nums {
            sum+=i;
            result.push(sum);
        }
        result
    }
}