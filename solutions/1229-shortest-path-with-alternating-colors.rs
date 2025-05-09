use std::collections::VecDeque;
use std::iter::{once, repeat};

impl Solution {
    pub fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut g = vec![[vec![], vec![]]; n];
        red_edges.into_iter().for_each(|v| g[v[0] as usize][0].push(v[1] as usize));
        blue_edges.into_iter().for_each(|v| g[v[0] as usize][1].push(v[1] as usize));
        println!("G: {:?}", g);
        let mut res = once([0, 0]).chain(repeat([n * 2, n * 2]).take(n - 1)).collect::<Vec<[usize; 2]>>();
        let mut bfs = VecDeque::from([[0, 0], [0, 1]]);
        while let Some([i, c]) = bfs.pop_front() {
            for j in g[i][c].iter() {
                if res[*j][c] == n * 2 {
                    res[*j][c] = res[i][1 - c] + 1;
                    bfs.push_back([*j, 1 - c]);
                }
            }
        }
        println!("BFS: {:?}", bfs);
        res.into_iter().map(|x| {
            let x = x[0].min(x[1]);
            if x < n * 2 {
                x as i32
            } else {
                -1
            }
        }).collect()
    }
}