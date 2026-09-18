impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let char = s.as_bytes();
        let mut score = 0;
        for i in 0..char.len() - 1 {
            let diff = (char[i] as i32 - char[i+1] as i32).abs();
            score += diff        
        } 
        score
    }
}