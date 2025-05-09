// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::once;

type OptRefNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lca_deepest_leaves(root: OptRefNode) -> OptRefNode {
        let mut paths = vec![vec![root.unwrap()]];
        let mut any_children = {
            let root = paths[0][0].borrow();
            root.left.is_some() || root.right.is_some()
        };
        while any_children {
            any_children = false;
            paths = paths.into_iter()
                .filter_map(|mut p| {
                    let lowest = p.last().unwrap().clone();
                    let lowest_ref = lowest.borrow();
                    match (lowest_ref.left.as_ref(), lowest_ref.right.as_ref()) {
                        (None, None) => {
                            None
                        }
                        (Some(x), None) | (None, Some(x)) => {
                            let x_ref = x.borrow();
                            if x_ref.left.is_some() || x_ref.right.is_some() {
                                any_children = true;
                            }

                            p.push(x.clone());
                            Some(vec![p])
                        }
                        (Some(l), Some(r)) => {
                            let l_ref = l.borrow();
                            let r_ref = r.borrow();
                            if l_ref.left.is_some() || l_ref.right.is_some() || r_ref.left.is_some() || r_ref.right.is_some() {
                                any_children = true;
                            }

                            let q = p.iter().map(|x| x.clone()).chain(once(r.clone())).collect();
                            p.push(l.clone());
                            Some(vec![p, q])
                        }
                    }
                })
                .flatten()
                .collect();
        }

        let n = paths[0].len();
        for i in (0..n).rev() {
            let first_i = &paths[0][i];
            if paths[1..].iter().all(|p| &p[i] == first_i) {
                return Some(first_i.clone());
            }
        }

        unreachable!()
    }
}