type Node = Rc<RefCell<TreeNode>>;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Node> {
        let pos_map = inorder.iter().enumerate().fold(HashMap::new(), |mut m, (i, n)| {
            m.insert(*n, i);
            m
        });
        Self::builder(&inorder, &postorder, &pos_map, &mut (postorder.len() - 1), 0, (inorder.len() - 1) as i32)
    }

    fn builder(inorder: &Vec<i32>, postorder: &Vec<i32>, map: &HashMap<i32, usize>, i: &mut usize, l: i32, h: i32) -> Option<Node> {
        if l > h {
            return None;
        }
        let mut n = TreeNode::new(postorder[*i]);
        *i -= 1;
        let m = *map.get(&n.val).unwrap() as i32;
        n.right = Self::builder(inorder, postorder, map, i, m + 1, h);
        n.left = Self::builder(inorder, postorder, map, i, l, m - 1);
        Some(Rc::new(RefCell::new(n)))
    }
}