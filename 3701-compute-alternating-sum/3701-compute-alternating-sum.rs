impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (i, num) in nums.iter().enumerate() {
            if i % 2 == 0 {
                ans += num;
            } else {
                ans -= num;
            }
        }
        ans
    }
}