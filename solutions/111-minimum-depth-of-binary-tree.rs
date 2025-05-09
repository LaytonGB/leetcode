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
type TN = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn min_depth(root: Option<TN>) -> i32 {
        if let Some(n) = root {
            Self::dfs(n, 1, i32::MAX)
        } else {
            0
        }
    }

    fn dfs(node: TN, curr_depth: i32, curr_min: i32) -> i32 {
        if curr_min < curr_depth {
            return curr_min;
        }
        
        let node = node.borrow();
        match (node.left.clone(), node.right.clone()) {
            (Some(n), None) | (None, Some(n)) => {
                Self::dfs(n, curr_depth + 1, curr_min)
                    .min(curr_min)
            }
            (Some(l), Some(r)) => {
                Self::dfs(l, curr_depth + 1, curr_min)
                    .min(Self::dfs(r, curr_depth + 1, curr_min))
                    .min(curr_min)
            }
            (None, None) => {
                curr_depth.min(curr_min)
            }
        }
    }
}