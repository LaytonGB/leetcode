use std::rc::Rc;
use std::cell::{RefCell, RefMut};
type CellNode = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn distribute_coins(root: Option<CellNode>) -> i32 {
        Self::dfs(root.as_ref()).0
    }

    fn dfs(node: Option<&CellNode>) -> (i32, i32) {
        node.map_or((0, 0), |node| {
            let bn = node.borrow();
            let (lc, ld) = Self::dfs(bn.left.as_ref());
            let (rc, rd) = Self::dfs(bn.right.as_ref());
            let d = ld + rd + bn.val - 1;
            let c = lc + rc + d.abs();
            (c, d)
        })
    }
}