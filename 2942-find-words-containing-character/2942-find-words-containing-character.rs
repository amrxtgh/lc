impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(words.len());
        for (i, str) in words.iter().enumerate() {
            if str.contains(x) {
                ans.push(i as i32)
            }
        }
        ans
    }
}