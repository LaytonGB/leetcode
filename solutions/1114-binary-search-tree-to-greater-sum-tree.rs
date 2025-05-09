use std::rc::Rc;
use std::cell::RefCell;
type TreeCell = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::h(root.as_ref(), 0).0.cloned()
    }

    fn h(root: Option<&TreeCell>, mut sum: i32) -> (Option<&TreeCell>, i32) {
        if let Some(r) = root {            
            let mut r_mut = r.borrow_mut();

            // right
            sum = Self::h(r_mut.right.as_ref(), sum).1;
            
            // this
            sum += r_mut.val;
            r_mut.val = sum;

            // left
            sum = Self::h(r_mut.left.as_ref(), sum).1;
        } else {
            return (None, sum);
        }
        (root, sum)
    }
}