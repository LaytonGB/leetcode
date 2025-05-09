use std::collections::*;
use std::iter::FromIterator;
impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let subordinates = manager.iter().enumerate().fold(vec![Vec::with_capacity(n as usize); n as usize], |mut v, (i,&m)| {
            if m != -1 {
                v[m as usize].push(i);
            }
            v
        });
        // println!("{:?}", subordinates);
        let mut res = 0;
        let mut q = subordinates[head_id as usize].iter().map(|&x| (x,inform_time[head_id as usize])).collect::<Vec<(usize,i32)>>();
        while let Some((idx,t)) = q.pop() {
            res = res.max(t);
            for &i in subordinates[idx].iter() {
                q.push((i, t + inform_time[idx]));
            }
        }
        res
    }
}