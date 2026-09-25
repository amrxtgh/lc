impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut good_pairs = 0;
        let n = nums1.len();
        let m = nums2.len();
        for i in 0..n {
            for j in 0..m {
                if (nums1[i] % (nums2[j]*k) == 0) {
                    good_pairs += 1;
                }
            }
        }
        good_pairs
    }
}