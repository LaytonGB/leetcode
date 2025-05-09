use std::rc::Rc;
use std::cell::RefCell;
use std::mem;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Solve::recover_tree(root)
    }
}

struct Solve {
    to_swap: [Option<Rc<RefCell<TreeNode>>>; 2],
    prev: Option<Rc<RefCell<TreeNode>>>,
}

impl Solve {
    pub fn recover_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
        let mut solver = Solve {
            to_swap: [None, None],
            prev: None,
        };

        solver.traverse(root);
        solver.swap_elements();
    }

    fn traverse(&mut self, node: &Option<Rc<RefCell<TreeNode>>>) {
        let Some(n) = node else {
            return;
        };
        let n = n.borrow();

        self.traverse(&n.left);

        if self.prev.as_ref().is_some_and(|p| p.borrow().val >= n.val) {
            if self.to_swap[0].is_none() {
                self.to_swap[0] = self.prev.clone();
            } 
            
            if self.to_swap[0].is_some() {
                self.to_swap[1] = node.clone();
            }
        }
        // println!("CURR: {:?} | PREV: {:?} | TO_SWAP: {:?}", n.val, self.prev.as_ref().unwrap_or(&Rc::new(RefCell::new(TreeNode::new(-1)))).borrow().val, self.to_swap.iter().map(|x| x.as_ref().unwrap_or(&Rc::new(RefCell::new(TreeNode::new(-1)))).borrow().val).collect::<Vec<_>>());
        self.prev = node.clone();

        self.traverse(&n.right);
    }

    fn swap_elements(&mut self) {
        let (Some(mut a), Some(mut b)) = (self.to_swap[0].as_ref(), self.to_swap[1].as_ref()) else {
            unreachable!()
        };

        let (mut a, mut b) = (a.borrow_mut(), b.borrow_mut());
        mem::swap(&mut a.val, &mut b.val);
    }
}