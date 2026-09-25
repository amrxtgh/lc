impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        (1..height.len())
        .filter(|&i| height[i-1] > threshold)
        .map(|i| i as i32)
        .collect()
    }
}