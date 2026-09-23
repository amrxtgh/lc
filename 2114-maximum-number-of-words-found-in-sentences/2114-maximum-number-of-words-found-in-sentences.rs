impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut word_count = 0;
        // let number = sentences.len();
        for sentence in sentences {
            let mut spaces = 0;
            for j in sentence.chars() {
                if j == ' ' {
                    spaces += 1;
                }
            }
            let words = spaces + 1;
            word_count = word_count.max(words);
        }
        word_count 
    }
}