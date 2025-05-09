use std::collections::*;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let map = edges.iter().fold(vec![Vec::new(); n], |mut m, e| {
            let (a, b) = (e[0] as usize, e[1] as usize);
            m[a].push(b);
            m[b].push(a);
            m
        });
        let mut visited = vec![false; n];
        let mut remaining = n;
        let mut res = 0;
        for i in 0..n {
            if visited[i] { continue; }
            let network_size = Self::dfs(&map, &mut visited, i);
            res += network_size * (remaining - network_size);
            remaining -= network_size;
        }
        res as i64
    }

    fn dfs(
        map: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        start: usize
    ) -> usize {
        visited[start] = true;
        let mut res = 1;
        for &c in map[start].iter() {
            if visited[c] { continue; }
            res += Self::dfs(map, visited, c);
        }
        res
    }
}