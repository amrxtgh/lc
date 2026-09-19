impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = Vec::with_capacity(2*n);
        for i in 0..(n*2)  {
            ans.push(nums[i % n]);
        }
        ans
    }
}