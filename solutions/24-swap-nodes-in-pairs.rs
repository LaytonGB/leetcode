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
use std::mem;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut n| {
            match n.next {
                Some(mut m) => {
                    n.next = Self::swap_pairs(m.next);
                    m.next = Some(n);
                    Some(m)
                }
                None => {
                    Some(n)
                }
            }
        })
    }
}