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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        if let Some(mut node) = root {
            let mut n = node.borrow();
            let mut l = Self::binary_tree_paths(n.left.clone());
            let mut r = Self::binary_tree_paths(n.right.clone());
            if l.len() == 0 && r.len() == 0 {
                return vec!(format!("{}", n.val));
            }
            l.append(&mut r);
            l.iter().map(|p| format!("{}->{}", n.val, p)).collect()
        } else {
            Vec::new()
        }
    }
}