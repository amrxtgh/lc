impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_wealth = 0;
        for customer in accounts {
            let wealth = customer.iter().sum();
            if wealth > max_wealth {
                max_wealth = wealth;
            }
        }
        max_wealth
    }
}