use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut q1 = VecDeque::new();
        let mut q2 = VecDeque::from([root]);
        while !q2.is_empty() {
            std::mem::swap(&mut q1, &mut q2);
            while let Some(node) = q1.pop_front() {
                if let Some(node) = node {
                    let node = node.borrow();
                    res = node.val;
                    q2.push_back(node.right.clone());
                    q2.push_back(node.left.clone());
                }
            }
        }
        res
    }
}