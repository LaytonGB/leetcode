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
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut node = &mut head;
        for _ in 0..left - 1 {
            node = &mut node.as_mut().unwrap().next;
        }

        Self::reverse_n(node, right - left + 1);

        head
    }

    fn reverse_n(head: &mut Option<Box<ListNode>>, n: i32) {
        let (mut prev, mut curr) = (None, head.take());
        for _ in 0..n {
            let Some(mut c) = curr else { panic!() };
            let next = c.next.take();
            c.next = prev;
            prev = Some(c);
            curr = next;
        }
        
        let mut new_last = &mut prev;
        while new_last.as_ref().unwrap().next.is_some() {
            new_last = &mut new_last.as_mut().unwrap().next;
        }
        new_last.as_mut().unwrap().next = curr;
        
        *head = prev.take();
    }
}