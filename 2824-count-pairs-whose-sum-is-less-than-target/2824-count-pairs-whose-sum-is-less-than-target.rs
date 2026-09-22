impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut c = 0;
        for i in 0..n {
            for j in (i+1)..n {
                if (nums[i] + nums[j] < target) {
                    c+=1;
                }
            }
        }
        c
    }
}