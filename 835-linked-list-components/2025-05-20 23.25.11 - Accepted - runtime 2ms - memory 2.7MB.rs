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
    pub fn num_components(mut head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = nums.into_iter().collect();
        
        let mut res = 0;
        while let Some(node) = head {
            if nums.contains(&node.val) && (node.next.is_none() || !nums.contains(&node.next.as_ref().unwrap().val)) {
                res += 1;
            }

            head = node.next;
        }

        res
    }
}