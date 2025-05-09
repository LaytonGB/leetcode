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
use std::collections::HashMap;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>, m: &mut HashMap<i32, i32>) {
            if let Some(n) = root {
                *m.entry(n.borrow().val).or_insert(0) += 1;
                count_nodes(&n.borrow().left, m);
                count_nodes(&n.borrow().right, m);
            }
        }
        
        let mut m = HashMap::new();
        count_nodes(&root, &mut m);
        let max = m.values().max().unwrap();
        m.iter().fold(Vec::new(), |mut res, (k,v)| {
            if v == max {
                res.push(*k);
            }
            res
        })
    }
}