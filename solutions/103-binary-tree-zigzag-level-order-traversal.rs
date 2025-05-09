use std::collections::VecDeque;
use std::mem::take;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return Vec::new();
        }

        let (mut q1, mut q2) = (VecDeque::new(), VecDeque::from([root.unwrap()]));
        let mut rev = false;
        let mut res = Vec::new();
        while !q2.is_empty() {
            q1 = take(&mut q2);
            q2 = VecDeque::new();
            res.push(Vec::new());
            while let Some(n) = q1.pop_front() {
                let n = n.borrow();
                res.last_mut().unwrap().push(n.val);
                if let Some(m) = &n.left {
                    q2.push_back(m.clone());
                }
                if let Some(m) = &n.right {
                    q2.push_back(m.clone());
                }
            }
            if rev {
                res.last_mut().unwrap().reverse();
            }
            rev = !rev;
        }
        res
    }
}