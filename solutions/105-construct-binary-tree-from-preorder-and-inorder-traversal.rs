use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::rec(&preorder, &inorder)
    }

    fn rec(pre: &[i32], ino: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.len() == 0 || ino.len() == 0 {
            return None;
        }

        let mut root = TreeNode::new(pre[0]);
        if let Some(idx) = ino.iter().position(|x| *x == pre[0]) {
            root.left = Self::rec(&pre[1..], &ino[..idx]);
            root.right = Self::rec(&pre[idx + 1..], &ino[idx + 1..]);
        }

        Some(Rc::new(RefCell::new(root)))
    }
}