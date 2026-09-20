impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut is_allowed = [false; 26];
        for b in allowed.bytes() {
            let index = (b - b'a') as usize;
            is_allowed[index] = true;
        }
        let mut consistent_count = 0;
        for word in words {
            let mut is_consistent = true;
            for b in word.bytes() {
                let index = (b - b'a') as usize;
                if !is_allowed[index] {
                    is_consistent = false;
                    break;
                }
            }
            if is_consistent {
                consistent_count += 1;
            }
        }
        consistent_count
    }
}