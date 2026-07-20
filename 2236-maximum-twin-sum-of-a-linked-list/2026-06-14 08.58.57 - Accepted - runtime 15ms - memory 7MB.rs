use std::collections::VecDeque;

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
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let mut q = VecDeque::new();
        while let Some(h) = head.take() {
            head = h.next;
            q.push_back(h.val);
        }

        let mut res = 0;
        while q.len() >= 2 {
            let a = q.pop_front().unwrap();
            let b = q.pop_back().unwrap();
            res = res.max(a + b);
        }

        res
    }
}