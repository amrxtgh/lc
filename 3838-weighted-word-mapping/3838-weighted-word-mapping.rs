use std::collections::HashMap;
impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut result = String::with_capacity(words.len());
        for word in words {
            let mut score = 0;
            for &byte in word.as_bytes() {
                let idx = (byte - b'a') as usize;
                score += weights[idx];
            }
            let remainder = (score % 26) as u8;
            let mapped_char = (b'z' - remainder) as char;
            result.push(mapped_char)
        }
        result
    }
}