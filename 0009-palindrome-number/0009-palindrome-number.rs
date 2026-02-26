impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut n = x;
        let mut reverse = 0;
        while n > 0 {
            let m = n % 10; // calculating last number
            reverse = reverse * 10 + m; 
            n = n/10;
        }
        x == reverse
    }
}