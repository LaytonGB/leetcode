use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        
        let mut l: Vec<Rc<RefCell<TreeNode>>> = Vec::with_capacity(1000);
        let mut r: Vec<Rc<RefCell<TreeNode>>> = Vec::with_capacity(1000);
        r.push(root);

        let mut res = 0;
        while !l.is_empty() || !r.is_empty() {
            if let Some(n) = l.pop() {
                let n = n.borrow();
                match (n.left.as_ref(), n.right.as_ref()) {
                    (None, None) => res += n.val,
                    (Some(n), None) => l.push(n.clone()),
                    (None, Some(n)) => r.push(n.clone()),
                    (Some(nl), Some(nr)) => {
                        l.push(nl.clone());
                        r.push(nr.clone());
                    }
                }
            }

            if let Some(n) = r.pop() {
                let n = n.borrow();
                match (n.left.as_ref(), n.right.as_ref()) {
                    (None, None) => (),
                    (Some(n), None) => l.push(n.clone()),
                    (None, Some(n)) => r.push(n.clone()),
                    (Some(nl), Some(nr)) => {
                        l.push(nl.clone());
                        r.push(nr.clone());
                    }
                }
            }
        }

        res
    }
}