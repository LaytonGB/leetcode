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
use std::collections::{HashMap, HashSet};

type Node = Rc<RefCell<TreeNode>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}
impl From<i32> for Side {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Right,
            1 => Self::Left,
            _ => panic!(),
        }
    }
}

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Node> {
        let mut map = HashMap::<i32, [i32; 2]>::new();
        let mut child_set = HashSet::<i32>::new();
        for d in descriptions {
            let (parent, child, side) = (d[0], d[1], d[2].into());
            map.entry(parent)
                .and_modify(|e| {
                    match side {
                        Side::Left => e[0] = child,
                        Side::Right => e[1] = child,
                    }
                })
                .or_insert(
                    match side {
                        Side::Left => [child, -1],
                        Side::Right => [-1, child],
                    }
                );
            child_set.insert(child);
        }

        let mut root = *map.keys()
            .find(|p| !child_set.contains(p))
            .unwrap();
        
        Self::build_tree(&map, root)
    }

    fn build_tree(map: &HashMap<i32, [i32; 2]>, value: i32) -> Option<Node> {
        let mut node = TreeNode::new(value);

        if let Some(children) = map.get(&value) {
            if children[0] != -1{
                node.left = Self::build_tree(map, children[0]);
            }
            if children[1] != -1 {
                node.right = Self::build_tree(map, children[1]);
            }
        }

        Some(Rc::new(RefCell::new(node)))
    }
}