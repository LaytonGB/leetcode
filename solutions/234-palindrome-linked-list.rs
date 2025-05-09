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
    fn collect(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = Vec::new();
        loop {
            match head {
                None => break,
                Some(node) => {
                    res.push(node.val);
                    head = node.next;
                }
            }
        }
        res
    }
    
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let v = Self::collect(head);
        let n = v.len() / 2;
        for i in 0..n {
            if v[i] != v[v.len() - i - 1] {
                return false;
            }
        }
        true
    }
}