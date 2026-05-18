// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while (list1.is_some() && list2.is_some()) {
            let take_list1 = list1.as_ref().unwrap().val < list2.as_ref().unwrap().val;
            let mut node = if take_list1 {
                list1.take().unwrap()
            } else {
                list2.take().unwrap()
            };
            if take_list1 {
                list1 = node.next.take();
            } else {
                list2 = node.next.take();
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy.next 
    }
}