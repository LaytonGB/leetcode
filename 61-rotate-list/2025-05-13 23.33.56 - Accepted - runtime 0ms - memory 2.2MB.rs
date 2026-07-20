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
            return head;
        }
        
        let n = Self::get_len(&head);
        let k = k as usize % n;

        if k == 0 {
            return head;
        }

        // Get node the comes before section to be rotated
        let target_node_position = n - k - 1;
        let mut pre = &mut head;
        for _ in 0..target_node_position {
            pre = &mut pre.as_mut().unwrap().next;
        }
        
        // Detach remainder
        let mut post = pre.as_mut().unwrap().next.take();
        
        // Attach remainder at start
        let target_post_position = k - 1;
        let mut last_post = &mut post;
        for _ in 0..target_post_position {
            last_post = &mut last_post.as_mut().unwrap().next;
        }
        last_post.as_mut().unwrap().next = head;
        
        // Return start of remainder
        post
    }

    fn get_len(head: &Option<Box<ListNode>>) -> usize {
        let mut res = 0;
        let mut node = head;
        while let Some(n) = node.as_ref() {
            res += 1;
            node = &n.next;
        }
        res
    }
}