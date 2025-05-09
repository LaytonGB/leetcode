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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        
        let mut v = Vec::new();
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }

        let n = v.len();
        v.rotate_right(k as usize % n);
        let mut dummy = None;
        while let Some(val) = v.pop() {
            let mut next = ListNode::new(val);
            next.next = dummy;
            dummy = Some(Box::new(next));
        }

        dummy
    }
}