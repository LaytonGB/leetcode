use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let Some(root) = root else {
            return Vec::new();
        };

        let mut layers = vec![vec![root]];
        let mut new_layer_added = true;
        while new_layer_added {
            new_layer_added = false;
            let mut next_layer = Vec::new();
            for node in layers.last().unwrap() {
                let n = node.borrow();
                match (n.left.as_ref(), n.right.as_ref()) {
                    (Some(l), Some(r)) => {
                        next_layer.push(l.clone());
                        next_layer.push(r.clone());
                    }
                    (Some(next), _) | (_, Some(next)) => {
                        next_layer.push(next.clone());
                    }
                    _ => (),
                }
            }
            if !next_layer.is_empty() {
                layers.push(next_layer);
                new_layer_added = true;
            }
        }

        layers.iter().rev().map(|v| v.iter().map(|n| n.borrow().val).collect()).collect()
    }
}