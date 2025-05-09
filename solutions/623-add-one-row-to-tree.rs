use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {val, left: root, right: None})));
        } else if let Some(root) = root.as_ref() {
            Self::recurse(root, val, depth - 1);
        }
        root
    }

    fn recurse(node: &Rc<RefCell<TreeNode>>, val: i32, depth: i32) {
        if depth <= 1 {
            let mut node = node.borrow_mut();
            let (left, right) = (node.left.take(), node.right.take());
            node.left = Some(Rc::new(RefCell::new(TreeNode {val, left, right: None})));
            node.right = Some(Rc::new(RefCell::new(TreeNode {val, left: None, right})));
        } else {
            let node = node.borrow();
            match (node.left.as_ref(), node.right.as_ref()) {
                (Some(l), Some(r)) => {
                    Self::recurse(l, val, depth - 1);
                    Self::recurse(r, val, depth - 1);
                }
                (Some(n), _) | (_, Some(n)) =>
                    Self::recurse(n, val, depth - 1),
                _ => (),
            }
        }
    }
}