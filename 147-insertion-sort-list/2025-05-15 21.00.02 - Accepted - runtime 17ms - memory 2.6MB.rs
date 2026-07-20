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
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {        
        // Set up output sentinel
        let mut res = Some(Box::new(ListNode::new(i32::MIN)));
        {
            let mut r = res.as_mut().unwrap();
            r.next = head.take();
            head = r.next.take();
        }

        // Iterate input and add to output as appropriate
        while head.is_some() {
            let val = head.as_ref().unwrap().val;
            let mut node = &mut res;
            while node.as_ref().unwrap().next.as_ref().is_some_and(|n| val > n.val) {
                node = &mut node.as_mut().unwrap().next;
            }
            let n = node.as_mut().unwrap();
            let next = n.next.take();
            n.next = head.take();
            head = n.next.as_mut().unwrap().next.take();
            n.next.as_mut().unwrap().next = next;
        }

        res.unwrap().next
    }
}