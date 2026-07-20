use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let del_set = to_delete.into_iter()
            .fold([false; 1001], |mut res, d| {
                res[d as usize] = true;
                res
            });
        
        let mut res = Vec::new();
        let mut q = VecDeque::new();
        q.push_back((root, true));
        
        while let Some(x) = q.pop_front() {
            let (Some(node), new_tree) = x else {
                continue;
            };
            let mut n = node.borrow_mut();
            let del_node = del_set[n.val as usize];
            if del_node {
                q.push_back((n.left.take(), true));
                q.push_back((n.right.take(), true));
            } else {
                if let Some(left) = n.left.as_ref() {
                    let l = left.borrow();
                    let del_l = del_set[l.val as usize];
                    drop(l);
                    if del_l {
                        q.push_back((n.left.take(), false));
                    } else {
                        q.push_back((n.left.clone(), false));
                    }
                }
                if let Some(right) = n.right.as_ref() {
                    let r = right.borrow();
                    let del_r = del_set[r.val as usize];
                    drop(r);
                    if del_r {
                        q.push_back((n.right.take(), false));
                    } else {
                        q.push_back((n.right.clone(), false));
                    }
                }
                
                if new_tree {
                    drop(n);
                    res.push(Some(node));
                }
            }
        }

        res
    }
}