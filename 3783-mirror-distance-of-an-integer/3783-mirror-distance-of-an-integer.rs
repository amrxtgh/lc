impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let s = n.abs().to_string();
        let reversed_s : String = s.chars().rev().collect();
        let reversed_n = reversed_s.parse().unwrap_or(0);
        (n - reversed_n).abs()
    }
}