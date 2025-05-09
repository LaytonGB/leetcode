use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut smallest = None;
        Self::dfs(root.as_ref().unwrap(), &mut Vec::new(), &mut smallest);
        // println!("{:?}", smallest);
        smallest.unwrap()
            .iter()
            .rev()
            .map(|x| char::from_u32((*x as u8 + b'a') as u32).unwrap())
            .collect()
    }

    fn dfs(node: &Rc<RefCell<TreeNode>>, builder: &mut Vec<i32>, smallest: &mut Option<Vec<i32>>) {        
        let n = node.borrow();
        builder.push(n.val);
        match (n.left.as_ref(), n.right.as_ref()) {
            (Some(l), Some(r)) => {
                Self::dfs(l, builder, smallest);
                Self::dfs(r, builder, smallest);
            }
            (Some(n), _) | (_, Some(n))
                => Self::dfs(n, builder, smallest),
            _ => {
                // println!("SMALLEST: {:?} | BUILDER: {:?}", smallest, builder);
                if !smallest.as_ref().is_some_and(|s| {
                    for (s, b) in s.iter().rev().zip(builder.iter().rev()) {
                        if s != b {
                            return s < b;
                        }
                    }
                    s.len() < builder.len()
                }) {
                    *smallest = Some(builder.clone());
                }
            }
        }
        builder.pop();
    }
}