use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut res = 0;
        Self::find_pairs_dfs(&mut res, distance, root.as_ref());
        res
    }

    fn find_pairs_dfs(
        pairs: &mut i32,
        distance: i32,
        node: Option<&Rc<RefCell<TreeNode>>>
    ) -> Vec<i32> {
        let Some(node) = node else {
            return vec![];
        };
        let n = node.borrow();
        let mut left = Self::find_pairs_dfs(pairs, distance, n.left.as_ref());
        let mut right = Self::find_pairs_dfs(pairs, distance, n.right.as_ref());
        match (left.is_empty(), right.is_empty()) {
            (true, true) => vec![1],
            (true, false) => {
                right.iter_mut().for_each(|dist| *dist += 1);
                right
            }
            (false, true) => {
                left.iter_mut().for_each(|dist| *dist += 1);
                left
            }
            (false, false) => {
                for l in left.iter() {
                    for r in right.iter() {
                        if l + r <= distance {
                            *pairs += 1;
                        }
                    }
                }
                left.append(&mut right);
                left.iter_mut().for_each(|dist| *dist += 1);
                left
            }
        }
    }
}