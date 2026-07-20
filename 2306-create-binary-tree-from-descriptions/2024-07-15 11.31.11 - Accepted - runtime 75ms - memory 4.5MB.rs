use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut children = HashSet::new();
        let mut map = HashMap::new();

        for (p, c, idx) in descriptions.into_iter().map(|d| (d[0], d[1], (d[2] == 0) as usize)) {
            children.insert(c);
            let mut v = map.entry(p).or_insert([None, None]);
            v[idx] = Some(c);
        }

        let root_val = *map.keys().find(|k| !children.contains(k)).unwrap();

        Self::construct(&map, root_val)
    }

    fn construct(map: &HashMap<i32, [Option<i32>; 2]>, root_val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = TreeNode::new(root_val);
        match map.get(&root_val) {
            Some([Some(l), Some(r)]) => {
                root.left = Self::construct(map, *l);
                root.right = Self::construct(map, *r);
            }
            Some([Some(l), None]) => root.left = Self::construct(map, *l),
            Some([None, Some(r)]) => root.right = Self::construct(map, *r),
            _ => {}
        }
        Some(Rc::new(RefCell::new(root)))
    }
}