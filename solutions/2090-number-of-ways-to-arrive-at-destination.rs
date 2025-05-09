use std::cmp::Reverse;
use std::collections::BinaryHeap;

const MOD: i64 = 10_i64.pow(9) + 7;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        
        let mut graph = roads.iter().fold(vec![vec![]; n], |mut res, r| {
            let [u, v, t] = &r[..] else { unreachable!() };
            res[*u as usize].push((*t as usize, *v as usize));
            res[*v as usize].push((*t as usize, *u as usize));
            res
        });

        let mut dist = vec![usize::MAX; n];
        dist[0] = 0;
        
        let mut ways = vec![0; n];
        ways[0] = 1;

        let mut q = BinaryHeap::from([(Reverse(0), 0)]);

        while let Some((Reverse(t1), u)) = q.pop() {
            if t1 > dist[u] {
                continue;
            }

            graph[u].iter().for_each(|(t2, v)| {
                if dist[u] + t2 < dist[*v] {
                    dist[*v] = dist[u] + t2;
                    ways[*v] = ways[u];
                    q.push((Reverse(dist[*v]), *v));
                } else if dist[u] + t2 == dist[*v] {
                    ways[*v] = (ways[u] + ways[*v]) % MOD;
                }
            });
        }

        ways[n - 1] as i32
    }
}