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
use std::collections::VecDeque;
impl Solution {
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut deq = VecDeque::new();
        let mut node = head.take();
        while let Some(mut n) = node {
            node = n.next.take();
            deq.push_back(n);
        }
        
        let mut dummy = ListNode::new(0);
        let mut ptr = &mut dummy;
        let mut front = true;
        while !deq.is_empty() {
            ptr.next = match front {
                true => deq.pop_front(),
                false => deq.pop_back(),
            };
            front = !front;
            ptr = ptr.next.as_mut().unwrap();
        }

        *head = dummy.next;
    }
}