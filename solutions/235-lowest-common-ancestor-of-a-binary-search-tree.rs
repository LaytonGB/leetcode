use std::rc::Rc;
use std::cell::RefCell;

use std::cmp::Ordering;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        let (p_val, q_val) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
        let mut root = root;
        while let Some(node) = root {
            let mut n = node.borrow_mut();
            match (n.val.cmp(&p_val), n.val.cmp(&q_val)) {
                (Ordering::Less, Ordering::Less) => {
                    root = n.right.take();
                }
                (Ordering::Greater, Ordering::Greater) => {
                    root = n.left.take();
                }
                (_, _) => {
                    return Some(Rc::new(RefCell::new(TreeNode::new(n.val))));
                }
            }
        }
        unreachable!()
    }
}