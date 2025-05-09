use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = vec![0];
        Self::diameter(&mut res, &root);
        res[0]
    }

    fn diameter(mut res: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();

            let left = Self::diameter(&mut res, &node.left);
            let right = Self::diameter(&mut res, &node.right);

            res[0] = res[0].max(left + right);

            left.max(right) + 1
        } else {
            0
        }
    }
}