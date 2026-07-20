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
impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root = root.unwrap();
        let mut list = vec![];
        Self::populate_node_list(&mut list, &root);
        Self::build_bst(&list, 0, list.len())
    }

    fn build_bst(list: &Vec<i32>, l: usize, h: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l >= h {
            return None;
        }
        
        let m = (h + l) / 2;
        Some(Rc::new(RefCell::new(TreeNode{
            val: list[m],
            left: Self::build_bst(list, l, m),
            right: Self::build_bst(list, m+1, h),
        })))
    }

    fn populate_node_list(list: &mut Vec<i32>, root: &Rc<RefCell<TreeNode>>) {
        let r = root.borrow();
        if let Some(l) = &r.left {
            Self::populate_node_list(list, l);
        }
        list.push(r.val);
        if let Some(r) = &r.right {
            Self::populate_node_list(list, r);
        }
    }
}