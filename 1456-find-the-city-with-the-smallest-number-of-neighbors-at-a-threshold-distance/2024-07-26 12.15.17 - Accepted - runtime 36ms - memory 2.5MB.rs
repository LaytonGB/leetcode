// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/solutions/5537644/explanations-no-one-will-give-you-2-detailed-approaches-extremely-simple-and-effective
use std::{
    cmp::Reverse,
    collections::BinaryHeap
};
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        
        let map = edges.into_iter()
            .fold(vec![Vec::with_capacity(n); n], |mut m, e| {
                let (f, t, d) = (e[0] as usize, e[1] as usize, e[2]);

                m[f].push((t, d));
                m[t].push((f, d));

                m
            });

        (0..n).fold((n, 0), |(min_neighbors, res), i| {
            let neighbor_count = Self::get_neighbors_within_dist(&map, distance_threshold, i);
            if neighbor_count <= min_neighbors {
                (neighbor_count, i)
            } else {
                (min_neighbors, res)
            }
        }).1 as i32
    }

    fn get_neighbors_within_dist(map: &Vec<Vec<(usize, i32)>>, d_max: i32, start: usize) -> usize {
        let mut q = BinaryHeap::from([Reverse((0, start))]);
        let mut visited = [false; 100];
        let mut res = 0;

        while let Some(Reverse((d1, f))) = q.pop() {
            if visited[f] {
                continue;
            }

            visited[f] = true;
            res += 1;
            for (t, d2) in &map[f] {
                let d = d1 + *d2;
                if d <= d_max {
                    q.push(Reverse((d, *t)));
                }
            }
        }

        res
    }
}