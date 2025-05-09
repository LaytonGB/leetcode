use std::rc::Rc;
use std::cell::RefCell;
type Node = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn merge_trees(root1: Option<Node>, root2: Option<Node>) -> Option<Node> {
        fn rec(n1: &Option<Node>, n2: &Option<Node>) -> Option<Node> {
            match (n1, n2) {
                (Some(n1), Some(n2)) => {
                    let (n1, n2) = (n1.borrow(), n2.borrow());
                    let mut root = TreeNode::new(n1.val + n2.val);
                    root.left = rec(&n1.left, &n2.left);
                    root.right = rec(&n1.right, &n2.right);
                    Some(Rc::new(RefCell::new(root)))
                }
                (Some(n), None) | (None, Some(n)) => Some(n.clone()),
                (None, None) => None,
            }
        }
        rec(&root1, &root2)
    }
}