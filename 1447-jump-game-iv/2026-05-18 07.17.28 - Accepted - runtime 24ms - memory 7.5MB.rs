use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let tgt = n - 1;
        
        let mut map = arr.iter().enumerate()
            .fold(HashMap::<i32, VecDeque<usize>>::new(), |mut m, (i, x)| {
                m.entry(*x)
                    .and_modify(|e| e.push_back(i))
                    .or_insert(VecDeque::from([i]));
                m
            });
        
        let mut q = VecDeque::from([0]);
        let mut v = vec![false; n];
        let mut res = 0;
        while q.len() > 0 {
            let m = q.len();
            for _ in 0..m {
                let i = q.pop_front().unwrap();

                if v[i] {
                    continue;
                }
                v[i] = true;

                if i == tgt {
                    return res;
                }
                
                if i + 1 < n {
                    q.push_back(i + 1);
                }
                if i - 1 < n {
                    q.push_back(i - 1);
                }

                // VecDeque consumes the appended values
                // This stops large lists from being repeatedly searched
                q.append(map.get_mut(&arr[i]).unwrap());
            }
            res += 1;
        }

        unreachable!()
    }
}