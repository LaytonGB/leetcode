use std::cmp::Ordering as O;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut parent = (0..n).collect::<Vec<_>>();

        for e in edges.iter() {
            Self::union_by_rank(e[0], e[1], &mut parent);
        }

        Self::find(source, &mut parent) == Self::find(destination, &mut parent)
    }

    fn union_by_rank(x: i32, y: i32, parent: &mut Vec<i32>) {
        let root_x = Self::find(x, parent);
        let root_y = Self::find(y, parent);

        if root_x == root_y {
            return;
        }

        match root_x.cmp(&root_y) {
            O::Greater => parent[root_y as usize] = root_x,
            O::Equal => {
                parent[root_x as usize] = root_y;
                parent[root_y as usize] += 1;
            }
            O::Less => parent[root_x as usize] = root_y,
        }
    }

    fn find(x: i32, parent: &mut Vec<i32>) -> i32 {
        if parent[x as usize] != x {
            parent[x as usize] = Self::find(parent[x as usize], parent);
        }
        parent[x as usize]
    }
}