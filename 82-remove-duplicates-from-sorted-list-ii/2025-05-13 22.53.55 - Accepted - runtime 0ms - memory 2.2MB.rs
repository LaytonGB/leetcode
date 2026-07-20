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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(
            ListNode {
                val: i32::MIN,
                next: head,
            }
        ));

        let mut node = &mut head;
        while let Some(n) = node.as_mut() {
            if n.next.as_ref().is_some_and(|n1| n1.next.as_ref().is_some_and(|n2| n1.val == n2.val)) {
                let val = n.next.as_ref().unwrap().val;
                while n.next.as_ref().is_some_and(|n1| n1.val == val) {
                    n.next = n.next.as_mut().unwrap().next.take();
                }
            } else if n.next.is_some() {
                node = &mut node.as_mut().unwrap().next;
            } else {
                break;
            }
        }

        head.unwrap().next
    }
}