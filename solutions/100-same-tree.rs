use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (p, q) = (p.borrow(), q.borrow());
                p.val == q.val
                    && Self::is_same_tree(p.left.clone(), q.left.clone())
                    && Self::is_same_tree(p.right.clone(), q.right.clone())
            }
            (Some(_), None) | (None, Some(_)) => false,
            _ => true,
        }
    }
}