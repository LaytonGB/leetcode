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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nodes = vec![];
        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            nodes.push(head.take());
            head = next;
        }

        nodes.sort_unstable_by_key(|n| n.as_ref().unwrap().val);

        let mut res = None;
        while let Some(mut node) = nodes.pop() {
            node.as_mut().unwrap().next = res;
            res = node;
        }

        res
    }
}