use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return 0;
        };
        
        let mut res = 0;
        Self::recurse(&root, 0, &mut res);
        res
    }

    fn recurse(node: &Rc<RefCell<TreeNode>>, partial_sum: i32, res: &mut i32) {
        let node = node.borrow();
        match (node.left.as_ref(), node.right.as_ref()) {
            (Some(l), Some(r)) => {
                let t = partial_sum * 10 + node.val;
                Self::recurse(l, t, res);
                Self::recurse(r, t, res);
            }
            (Some(n), _) | (_, Some(n))
                => Self::recurse(n, partial_sum * 10 + node.val, res),
            _ => *res += partial_sum * 10 + node.val,
        };
    }
}