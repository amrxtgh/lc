impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
       if strs.is_empty() {
        return String::from("");
       } 
       let first_str = &strs[0];
       for (i, ch) in first_str.chars().enumerate() {
        for oth_str in &strs[1..] {
            if i >= oth_str.len() || oth_str.as_bytes()[i] != ch as u8 {
                return first_str[..i].to_string();
            }
        }
       }
    first_str.clone() 
    }
}