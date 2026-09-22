impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let n = encoded.len();

        let mut arr = Vec::new();
        arr.push(first);

        for i in 0..n {
            arr.push(encoded[i] ^ arr.last().unwrap());
            // arr[i + 1] = encoded[i] ^ arr[i];
        }
        arr
    }
}