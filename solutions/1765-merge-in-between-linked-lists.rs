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
    pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals = Vec::new();

        // get full list values
        for _ in 0..a {
            let node = list1.unwrap();
            vals.push(node.val);
            list1 = node.next;
        }

        while list2.is_some() {
            let node = list2.unwrap();
            vals.push(node.val);
            list2 = node.next;
        }

        for _ in a..=b {
            list1 = list1.unwrap().next;
        }

        while list1.is_some() {
            let node = list1.unwrap();
            vals.push(node.val);
            list1 = node.next;
        }

        // build list from stack
        let mut head = None;
        while let Some(val) = vals.pop() {
            let mut new_head = ListNode::new(val);
            new_head.next = head;
            head = Some(Box::new(new_head));
        }

        head
    }
}