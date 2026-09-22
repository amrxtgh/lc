impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let n = encoded.len();

        let mut arr = Vec::new();
        arr.push(first);

        encoded
        .iter()
        .for_each(|x| {
            arr.push(x ^ arr.last().unwrap())
        });

        arr
    }
}