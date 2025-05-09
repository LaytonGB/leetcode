use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::eval(root.as_ref())
    }

    fn eval(root: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let root = root.borrow();
        match root.val {
            0 => false,
            1 => true,
            x => {
                let l = Self::eval(root.left.as_ref());
                let r = Self::eval(root.right.as_ref());
                match x {
                    2 => l || r,
                    3 => l && r,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}