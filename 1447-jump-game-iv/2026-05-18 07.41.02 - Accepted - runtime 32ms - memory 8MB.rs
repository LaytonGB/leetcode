use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let tgt = n - 1;
        
        let map = arr.iter().enumerate()
            .fold(HashMap::<i32, VecDeque<usize>>::new(), |mut m, (i, x)| {
                m.entry(*x)
                    .and_modify(|e| e.push_back(i))
                    .or_insert(VecDeque::from([i]));
                m
            });
        
        let mut q = VecDeque::from([0]);
        let mut vis_idx = vec![false; n];
        let mut vis_val = HashSet::new();
        let mut res = 0;
        while q.len() > 0 {
            let m = q.len();
            for _ in 0..m {
                let i = q.pop_front().unwrap();

                if vis_idx[i] {
                    continue;
                }
                vis_idx[i] = true;

                if i == tgt {
                    return res;
                }
                
                if i + 1 < n {
                    q.push_back(i + 1);
                }
                if i - 1 < n {
                    q.push_back(i - 1);
                }

                // This stops large lists from being repeatedly searched
                if !vis_val.contains(&arr[i]) {
                    q.extend(map.get(&arr[i]).unwrap());
                    vis_val.insert(arr[i]);
                }
            }
            res += 1;
        }

        unreachable!()
    }
}