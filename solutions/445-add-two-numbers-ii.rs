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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut v1 = Vec::new();
        while let Some(n) = l1 {
            v1.push(n.val);
            l1 = n.next;
        }
        let mut v2 = Vec::new();
        while let Some(n) = l2 {
            v2.push(n.val);
            l2 = n.next;
        }

        let mut last = None;
        let mut carry = 0;
        while !v1.is_empty() || !v2.is_empty() || carry != 0 {
            let n1 = v1.pop().unwrap_or(0);
            let n2 = v2.pop().unwrap_or(0);
            let n3 = n1 + n2 + carry;
            let mut new_node = Box::new(ListNode::new(n3 % 10));
            carry = n3 / 10;

            new_node.next = last;
            last = Some(new_node);
        }

        last
    }
}