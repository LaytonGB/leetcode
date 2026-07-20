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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if x <= -100 || x > 100 {
            return head;
        }
        
        let mut low = Some(Box::new(
            ListNode::new(i32::MIN)
        ));
        let mut high = Some(Box::new(
            ListNode::new(i32::MIN)
        ));

        let mut l = &mut low;
        let mut h = &mut high;

        let mut node = head;
        while let Some(n) = node.as_mut() {
            if n.val < x {
                l.as_mut().unwrap().next = node;
                l = &mut l.as_mut().unwrap().next;
                node = l.as_mut().unwrap().next.take();
            } else {
                h.as_mut().unwrap().next = node;
                h = &mut h.as_mut().unwrap().next;
                node = h.as_mut().unwrap().next.take();
            }
        }

        l.as_mut().unwrap().next = high.unwrap().next;
        low.unwrap().next
    }
}