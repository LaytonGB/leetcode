use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        let map = Self::tree_into_map(root);
        let mut path = String::new();
        Self::dfs(&map, &mut path, None, start_value, dest_value);
        path
    }

    fn tree_into_map(root: Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, [Option<i32>; 3]> {
        let mut map = HashMap::new();
        let mut q = vec![root.unwrap()];
        while let Some(node) = q.pop() {
            let n = node.borrow();
            let (mut l_val, mut r_val) = (None, None);
            if let Some(left) = n.left.as_ref() {
                let l = left.borrow();
                l_val = Some(l.val);

                let mut l = map.entry(l.val)
                    .or_insert([None; 3]);
                l[0] = Some(n.val);

                q.push(left.clone());
            }
            if let Some(right) = n.right.as_ref() {
                let r = right.borrow();
                r_val = Some(r.val);

                let mut r = map.entry(r.val)
                    .or_insert([None; 3]);
                r[0] = Some(n.val);

                q.push(right.clone());
            }
            let mut n = map.entry(n.val)
                .or_insert([None; 3]);
            n[1] = l_val;
            n[2] = r_val;
        }
        // println!("{:?}", map);
        map
    }

    fn dfs(map: &HashMap<i32, [Option<i32>; 3]>, path: &mut String, last: Option<i32>, curr: i32, dest: i32) -> bool {
        if curr == dest {
            return true;
        }
        
        for [u, l, r] in map.get(&curr) {
            if let Some(u) = u {
                if !last.is_some_and(|last| last == *u) {
                    path.push('U');
                    if Self::dfs(map, path, Some(curr), *u, dest) {
                        return true;
                    }
                    path.pop();
                }
            }
            if let Some(l) = l {
                if !last.is_some_and(|last| last == *l) {
                    path.push('L');
                    if Self::dfs(map, path, Some(curr), *l, dest) {
                        return true;
                    }
                    path.pop();
                }
            }
            if let Some(r) = r {
                if !last.is_some_and(|last| last == *r) {
                    path.push('R');
                    if Self::dfs(map, path, Some(curr), *r, dest) {
                        return true;
                    }
                    path.pop();
                }
            }
        }

        false
    }
}