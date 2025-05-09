type Node = Rc<RefCell<TreeNode>>;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;
impl Solution {
    pub fn is_symmetric(root: Option<Node>) -> bool {
        if root.is_none() {
            return false;
        }
        let (mut s1, mut s2) = (Vec::new(), Vec::new());
        let (mut n1, mut n2) = (root.clone(), root);
        while !s1.is_empty() || !s2.is_empty() || n1.is_some() || n2.is_some() {
            if s1.is_empty() != s2.is_empty() {
                return false;
            }
            while n1.is_some() || n2.is_some() {
                if n1.is_some() != n2.is_some() {
                    return false;
                }
                let (m1, m2) = (n1.clone().unwrap(), n2.clone().unwrap());
                let (m1, m2) = (m1.borrow(), m2.borrow());
                s1.push(n1);
                s2.push(n2);
                n1 = m1.left.clone();
                n2 = m2.right.clone();
            }
            n1 = s1.pop().unwrap();
            n2 = s2.pop().unwrap();
            if n1.is_some() || n2.is_some() {
                if n1.is_some() != n2.is_some() {
                    return false;
                }
                let (m1, m2) = (n1.unwrap(), n2.unwrap());
                let (m1, m2) = (m1.borrow(), m2.borrow());
                if m1.val != m2.val {
                    return false;
                }
                n1 = m1.right.clone();
                n2 = m2.left.clone();
            }
        }
        true
    }
}