impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        for num in nums {
            if num % 2 == 0 {
                ans.push(0);
            } else {
                ans.push(1);
            } 
        }   
        ans.sort();
        ans
    }
}