impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            if c == '(' || c == '{' || c == '[' {
                stack.push(c);
            }
            else if c == ')' {
                if stack.is_empty() { return false; }
                let top = stack.pop().unwrap();
                if top != '(' {
                    return false;
                }
            }
            else if c == '}' {
                if stack.is_empty() { return false; }
                let top = stack.pop().unwrap();
                if top != '{' {
                    return false;
                }
            }
            else if c == ']' {
                if stack.is_empty() { return false; }
                let top = stack.pop().unwrap();
                if top != '[' {
                    return false;
                }
            }

        }
        stack.is_empty()
    }
}