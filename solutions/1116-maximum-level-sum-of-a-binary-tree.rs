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
use std::mem;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q2 = vec![root.unwrap()];
        let mut q1 = Vec::new();
        let mut res = -1;
        let mut max = i32::MIN;
        let mut i = 1;
        while !q2.is_empty() {
            let mut total = 0;
            mem::swap(&mut q1, &mut q2);
            while let Some(n) = q1.pop() {
                let n = n.borrow();
                total += n.val;
                if let Some(l) = n.left.clone() {
                    q2.push(l);
                }
                if let Some(r) = n.right.clone() {
                    q2.push(r);
                }
            }
            if total > max {
                res = i;
                max = total;
            }
            i += 1;
        }
        res
    }
}