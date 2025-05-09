use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn max_a_d(node: Option<&Rc<RefCell<TreeNode>>>, high: i32, low: i32) -> i32 {
        if let Some(n) = node.as_ref() {
            let h = high.max(n.borrow().val);
            let l = low.min(n.borrow().val);
            Self::max_a_d(n.borrow().left.as_ref(), h, l).max(Self::max_a_d(n.borrow().right.as_ref(), h, l))
        } else {
            (high - low).abs()
        }
    }

    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root_val = root.as_ref().unwrap().borrow().val;
        Self::max_a_d(root.as_ref(), root_val, root_val)
    }
}