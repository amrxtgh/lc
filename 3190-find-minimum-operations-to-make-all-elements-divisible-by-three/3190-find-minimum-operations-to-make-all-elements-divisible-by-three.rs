impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut min_op = 0;
        for i in nums {
            if i % 3 != 0 {
                min_op += 1;
            }
        }
        min_op
    }
}