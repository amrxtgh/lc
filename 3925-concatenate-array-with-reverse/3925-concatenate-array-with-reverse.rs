impl Solution {
    pub fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans : Vec<i32> = vec![0; 2*n];
        for i in 0..n {
            ans[i] = nums[i];
            ans[2*n-1-i] = nums[i];
        }
        ans
    }
}