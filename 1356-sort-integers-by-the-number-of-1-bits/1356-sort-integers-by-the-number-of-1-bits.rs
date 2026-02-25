impl Solution {
    pub fn count_ones(mut n: u32) -> u32 {
        let mut count = 0;
        while n > 0 {
            if n & 1 == 1 {
                count += 1;
            }
            n >>= 1;
        } 
        count
    }
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
       let len = arr.len();
       for i in 0..len {
        for j in i+1..len {
            let ones_i = Self::count_ones(arr[i] as u32);
            let ones_j = Self::count_ones(arr[j] as u32);
            if ones_i > ones_j || (ones_i == ones_j && arr[i] > arr[j]) {
                arr.swap(i, j);
            }
        }
       }
       arr
    }
}