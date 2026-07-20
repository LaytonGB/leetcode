use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

struct FindElements {
    values: HashSet<i32>,
}

impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut set = HashSet::new();
        Self::insert_values(&mut set, &root, 0);
        Self {
            values: set,
        }
    }

    fn insert_values(set: &mut HashSet<i32>, node: &Option<Rc<RefCell<TreeNode>>>, value: i32) {
        let Some(rc) = node.as_ref() else {
            return;
        };

        set.insert(value);

        let node = rc.borrow();
        let left = &node.left;
        let right = &node.right;

        Self::insert_values(set, left, value * 2 + 1);
        Self::insert_values(set, right, value * 2 + 2);
    }
    
    fn find(&self, target: i32) -> bool {
        return self.values.contains(&target);
    }
}
