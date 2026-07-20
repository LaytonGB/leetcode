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
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];
        let mut i = 0;
        while let Some(node) = head {
            res.push(0);

            while stack.last().is_some_and(|(_, l)| node.val > *l) {
                let (i, _) = stack.pop().unwrap();
                res[i] = node.val;
            }

            stack.push((i, node.val));
            i += 1;

            head = node.next;
        }

        res
    }
}