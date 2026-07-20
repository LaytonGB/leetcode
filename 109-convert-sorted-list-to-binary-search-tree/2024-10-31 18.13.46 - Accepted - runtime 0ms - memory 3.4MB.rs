// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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

pub trait IntoVec {
    fn to_vec(self) -> Vec<i32>;
}

impl IntoVec for Option<Box<ListNode>> {
    fn to_vec(self) -> Vec<i32> {
        let mut curr = self;
        let mut res = vec![];
        while curr.is_some() {
            let mut c = curr.take().unwrap();
            res.push(c.val);
            curr = c.next;
        }
        res
    }
}

pub trait IntoTree {
    fn to_tree(&self) -> Option<Rc<RefCell<TreeNode>>>;
}

impl IntoTree for &[i32] {
    fn to_tree(&self) -> Option<Rc<RefCell<TreeNode>>> {
        let n = self.len();
        if n == 0 {
            return None;
        } else if n == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(self[0]))));
        }

        let mut m = TreeNode::new(self[n / 2]);
        let l = self[..n / 2].as_ref().to_tree();
        let r = if n / 2 + 1 < n {
            self[n / 2 + 1..].as_ref().to_tree()
        } else {
            None
        };

        m.left = l;
        m.right = r;
        Some(Rc::new(RefCell::new(m)))
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut list = head.to_vec();
        // println!("{:?}", list);
        list.as_slice().as_ref().to_tree()
    }
}