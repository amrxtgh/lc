impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut result = String::new();
        let mut word_count = 0;
        for i in s.chars() {
            if i == ' ' {
                word_count += 1;
                if word_count == k { break; }
            }
            result.push(i);
        }
        result
    }
}