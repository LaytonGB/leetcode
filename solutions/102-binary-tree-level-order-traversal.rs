use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q1 = VecDeque::new();
        let mut q2 = VecDeque::new();
        let mut res = Vec::new();
        let mut rows = usize::MAX;
        let mut r;
        q1.push_back(root); // start with root node
        while !q1.is_empty() || !q2.is_empty() {
            rows += 1;
            r = Vec::new();
            while let Some(node) = q1.pop_front() {
                if let Some(n) = node {
                    r.push(n.borrow().val.clone());
                    q2.push_back(n);
                }
            }
            if q2.len() > 0 {
                res.push(r);
            }
            while let Some(n) = q2.pop_front() {
                q1.push_back(n.borrow().left.clone());
                q1.push_back(n.borrow().right.clone());
            }
        }
        res
    }
}