use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;
use std::mem::swap;
impl Solution {
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() { return false; }
        let (mut q1, mut q2) = (VecDeque::new(), VecDeque::new());
        q2.push_back(root.unwrap());
        let mut found_missing = false;
        while !q2.is_empty() {
            swap(&mut q1, &mut q2);
            while let Some(n) = q1.pop_front() {
                let n = n.borrow();
                if let Some(l) = n.left.clone() {
                    if found_missing {
                        return false;
                    }
                    q2.push_back(l);
                } else {
                    found_missing = true;
                }
                if let Some(r) = n.right.clone() {
                    if found_missing {
                        return false;
                    }
                    q2.push_back(r);
                } else {
                    found_missing = true;
                }
            }
        }
        true
    }
}