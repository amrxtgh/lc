use std::collections::HashSet;
impl Solution {
    pub fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32> {
        let friend_set : HashSet<i32> = friends.into_iter().collect();
        order.into_iter().filter(|id| friend_set.contains(id)).collect()
    }
}