use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = Vec::new();
        let mut node = root;
        while node.is_some() || !stack.is_empty() {
            while node.is_some() {
                stack.push(node.clone());
                let n = node.unwrap();
                let n = n.borrow();
                res.push(n.val);
                node = n.right.clone();
            }
            node = stack.pop().unwrap();
            let n = node.unwrap();
            let n = n.borrow();
            node = n.left.clone();
        }
        res.into_iter().rev().collect()
    }
}