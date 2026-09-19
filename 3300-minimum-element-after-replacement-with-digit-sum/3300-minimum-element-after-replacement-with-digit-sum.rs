impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter().map(Self::digit_sum).min().unwrap_or(0)
    }
    fn digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        sum
    }
}