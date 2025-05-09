use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root:Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root.as_ref(), i64::MIN, i64::MAX)
    }

    fn helper(root:Option<&Rc<RefCell<TreeNode>>>, min:i64, max:i64) -> bool {
        if let Some(n) = root {
            let n = n.borrow();
            if n.val as i64 <= min || n.val as i64 >= max { return false; }
            Self::helper(n.left.as_ref(), min, n.val as i64) && Self::helper(n.right.as_ref(), n.val as i64, max)
        } else {
            true
        }
    }
}