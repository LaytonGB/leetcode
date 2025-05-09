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

type RefNode = Rc<RefCell<TreeNode>>;
use std::collections::HashMap;
impl Solution {
    pub fn pseudo_palindromic_paths (root: Option<RefNode>) -> i32 {
        Self::count_palindromes(&mut HashMap::new(), root.unwrap())
    }

    fn count_palindromes(num_counts: &mut HashMap<i32, usize>, node: RefNode) -> i32 {
        let node = node.borrow();
        num_counts.entry(node.val)
            .and_modify(|val| *val += 1)
            .or_insert(1);

        let res = if node.left.is_some() || node.right.is_some() {
            node.left.clone().and_then(|left| Some(Self::count_palindromes(num_counts, left))).unwrap_or(0) +
                node.right.clone().and_then(|right| Some(Self::count_palindromes(num_counts, right))).unwrap_or(0)
        } else if Self::is_palindrome(&num_counts) {
            1
        } else {
            0
        };

        num_counts.entry(node.val)
            .and_modify(|val| *val -= 1);

        res
    }

    fn is_palindrome(num_counts: &HashMap<i32, usize>) -> bool {
        let mut odd_num_found = false;
        num_counts.values().all(|count| {
            if count % 2 == 1 {
                if odd_num_found {
                    return false;
                } else {
                    odd_num_found = true;
                }
            }
            true
        })
    }
}