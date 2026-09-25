impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let str1 = word1.concat();
        let str2 = word2.concat();
        str1 == str2
    }
}