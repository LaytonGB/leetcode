use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(&root)
    }

    fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                1 + Self::helper(&n.borrow().left)
                    .max(Self::helper(&n.borrow().right))
            }
        }
    }
}