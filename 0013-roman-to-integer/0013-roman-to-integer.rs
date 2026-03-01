use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let values = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);        
        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        for i in 0..chars.len() {
            let curr = values[&chars[i]];

            if i + 1 < chars.len() {
                let next = values[&chars[i + 1]];
                if curr < next {
                    total -= curr;
                } else {
                    total += curr;
                }
            } else {
                total += curr;
            }
        }
        total
    }
}