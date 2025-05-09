use std::collections::*;
impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let det_map = bombs.iter().enumerate().fold(vec![vec![]; bombs.len()], |mut v, (i, b)| {
            if let [x1,y1,r1] = b[..] {
                let w = bombs.iter().enumerate().filter_map(|(j, b)| {
                    if let [x2,y2,r2] = b[..] {
                        if (((x2 - x1) as f64).powi(2) + ((y2 - y1) as f64).powi(2)).sqrt() <= r1 as f64 {
                            Some(j)
                        } else { None }
                    } else { None }
                }).collect();
                v[i] = w;
            }
            v
        });
        println!("{:?}", det_map);
        let mut res = 1;
        for idx in 0..bombs.len() {
            res = res.max(Self::bfs(&det_map, idx));
            println!();
        }
        res
    }

    fn bfs(map: &Vec<Vec<usize>>, idx: usize) -> i32 {
        let mut q = VecDeque::from(map[idx].clone());
        let mut v = vec![false; map.len()];
        let mut res = 0;
        while let Some(i) = q.pop_front() {
            if v[i] {
                continue;
            }
            print!("{} ", i);
            v[i] = true;
            res += 1;
            for j in map[i].iter() {
                q.push_back(*j);
            }
        }
        res
    }
}