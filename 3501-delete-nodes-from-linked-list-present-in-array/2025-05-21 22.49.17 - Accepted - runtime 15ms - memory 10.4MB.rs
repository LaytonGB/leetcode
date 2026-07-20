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
use std::collections::HashSet;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums: HashSet<i32> = nums.into_iter().collect();
        
        let mut res = Some(Box::new(
            ListNode {
                val: -1,
                next: head,
            }
        ));
        let mut node = res.as_mut().unwrap();

        while let Some(next) = node.next.as_mut() {
            if nums.contains(&next.val) {
                node.next = next.next.take();
            } else {
                node = node.next.as_mut().unwrap();
            }
        }

        res.unwrap().next
    }
}