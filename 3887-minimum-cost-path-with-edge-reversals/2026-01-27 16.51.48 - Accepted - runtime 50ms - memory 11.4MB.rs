use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut map = vec![Vec::with_capacity(4); n];
        for e in edges {
            let (a, b, dist) = (e[0] as usize, e[1] as usize, e[2]);
            map[a].push((b as u32, dist));
            map[b].push((a as u32, dist * 2));
        }

        let mut vis = vec![false; n];
        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0_u32));
        while let Some((Reverse(d1), a)) = heap.pop() {
            let a = a as usize;

            if a == n - 1 {
                return d1;
            }

            if vis[a] {
                continue;
            }
            vis[a] = true;

            for &(b_u32, d2) in &map[a] {
                let d3 = d1 + d2;
                let b = b_u32 as usize;
                if vis[b] || dist[b] <= d3 {
                    continue;
                }

                dist[b] = d3;
                heap.push((Reverse(d3), b_u32));
            }
        }

        -1
    }
}