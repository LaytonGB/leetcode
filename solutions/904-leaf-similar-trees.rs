
// #[derive(Debug)]
enum LastMove {
    None,
    UpFromLeft,
    UpFromRight,
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut path1 = vec![root1.unwrap()];
        let mut path2 = vec![root2.unwrap()];
        let mut leaf1: Option<Rc<RefCell<TreeNode>>>;
        let mut leaf2: Option<Rc<RefCell<TreeNode>>>;
        loop {
            (leaf1, leaf2) = (Self::get_next_leaf(&mut path1), Self::get_next_leaf(&mut path2));
            // println!("{:?} :: {:?}", path1, path2);
            // println!("{:?} :: {:?}", leaf1, leaf2);
            if leaf1.is_none() || leaf2.is_none() {
                break;
            }
            let (leaf1, leaf2) = (leaf1.unwrap(), leaf2.unwrap());
            if leaf1.borrow().val != leaf2.borrow().val {
                return false;
            }
        }
        leaf1.is_some() == leaf2.is_some()
    }

    fn get_next_leaf(path: &mut Vec<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(node) = path.pop() {
            let n = node.borrow();
            if let Some(l) = &n.left {
                path.push(l.clone());
            }
            if let Some(r) = &n.right {
                path.push(r.clone());
            }
            if n.left.is_none() && n.right.is_none() {
                drop(n);
                return Some(node);
            }
        }
        None
    }
}