use std::cmp::Ordering as O;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut parents: Vec<usize> = (0..n).collect();
        let mut node_counts = vec![0; n];

        edges.iter().for_each(|e| {
            Self::unite(&mut parents, e[0] as usize, e[1] as usize);
        });

        let mut root_map: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(n);
        let mut edge_counts: HashMap<usize, usize> = HashMap::with_capacity(n * n / 2);

        (0..n).for_each(|i| {
            let root = Self::get_root(&parents, i);
            root_map.entry(root)
                .and_modify(|node_set| { node_set.insert(i); })
                .or_insert(HashSet::from([i]));
        });

        edges.iter().for_each(|e| {
            let root = Self::get_root(&parents, e[0] as usize);
            edge_counts.entry(root)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        root_map.iter().fold(0_i32, |res, (root, nodes)| {
            let node_count = root_map.get(root).unwrap().len();
            res + (node_count * (node_count - 1) / 2 == *edge_counts.get(root).unwrap_or(&0)) as i32
        })
    }

    fn unite(parents: &mut [usize], a: usize, b: usize) {
        let a_root = Self::get_root(&parents, a);
        let b_root = Self::get_root(&parents, b);

        match (a_root.cmp(&b_root)) {
            O::Equal => {}
            O::Less => {
                parents[b_root] = a_root;
            }
            O::Greater => {
                parents[a_root] = b_root;
            }
        }
    }

    fn get_root(parents: &[usize], node: usize) -> usize {
        if (parents[node] == node) {
            node
        } else {
            Self::get_root(parents, parents[node])
        }
    }
}