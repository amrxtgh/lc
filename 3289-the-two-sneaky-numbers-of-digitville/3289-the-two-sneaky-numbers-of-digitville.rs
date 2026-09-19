use std::collections::HashSet;
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen  = HashSet::new();
        let mut result = Vec::with_capacity(2);

        for num in nums {
            if !seen.insert(num) {
                result.push(num);
            }
            if result.len() == 2 {
                break;
            }
        }
        result
    }
}