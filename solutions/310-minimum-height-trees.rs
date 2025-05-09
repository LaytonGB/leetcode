use std::collections::VecDeque;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        
        let (adj, mut degrees) = edges.into_iter()
            .fold((vec![Vec::with_capacity(n as usize); n as usize], vec![0; n as usize]),
            |(mut v, mut d), e| {
                v[e[0] as usize].push(e[1] as usize);
                v[e[1] as usize].push(e[0] as usize);
                d[e[0] as usize] += 1;
                d[e[1] as usize] += 1;
                (v, d)
            });

        let mut q = VecDeque::with_capacity(n as usize);
        for i in 0..degrees.len() {
            if degrees[i] == 1 {
                q.push_back(i);
            }
        }

        let mut res = Vec::with_capacity(n as usize);
        while !q.is_empty() {
            res.clear();
            let n = q.len();
            for _ in 0..n {
                let node = q.pop_front().unwrap();
                res.push(node as i32);
                for &c in adj[node].iter() {
                    degrees[c] -= 1;
                    
                    if degrees[c] == 1 {
                        q.push_back(c);
                    }
                }
            }
        }

        res
    }
}