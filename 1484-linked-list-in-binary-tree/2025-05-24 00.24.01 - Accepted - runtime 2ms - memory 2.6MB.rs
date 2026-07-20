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
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let list = Self::get_list(head);
        Self::dfs(root, &list, 0)
    }

    fn get_list(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = Vec::with_capacity(100);
        while let Some(h) = head {
            res.push(h.val);
            head = h.next;
        }
        res
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, list: &[i32], index: usize) -> bool {
        if index == list.len() {
            return true;
        }

        match root {
            Some(node) => {
                let b = node.borrow();
                
                if b.val == list[index] {
                    if Self::dfs(b.left.clone(), list, index + 1)
                    || Self::dfs(b.right.clone(), list, index + 1) {
                        return true;
                    }
                }
                
                if index > 0 {
                    return false;
                } else {
                    Self::dfs(b.left.clone(), list, index)
                    || Self::dfs(b.right.clone(), list, index)
                }
            }
            None => false,
        }
    }
}