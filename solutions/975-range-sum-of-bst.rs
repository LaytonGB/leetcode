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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(n) = root {
            let val = n.borrow().val;
            let mut sum = 0;
            if val >= low && val <= high {
                sum += val;
            }
            if val > low && val < high {
                sum += Self::range_sum_bst(n.borrow().left.clone(), low, high);
                sum += Self::range_sum_bst(n.borrow().right.clone(), low, high);
            } else if val <= low {
                sum += Self::range_sum_bst(n.borrow().right.clone(), low, high);
            } else { // val >= high
                sum += Self::range_sum_bst(n.borrow().left.clone(), low, high);
            }
            sum
        } else {
            0
        }
    }
}