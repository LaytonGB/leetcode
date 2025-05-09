use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r_ref) = root.as_ref() {
            let mut r = r_ref.borrow_mut();

            r.left = Self::remove_leaf_nodes(r.left.take(), target);
            r.right = Self::remove_leaf_nodes(r.right.take(), target);

            if r.left.is_none() && r.right.is_none() && r.val == target {
                return None;
            }
        }

        root
    }
}