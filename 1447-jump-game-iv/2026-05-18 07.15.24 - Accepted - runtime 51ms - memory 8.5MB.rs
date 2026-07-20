use std::collections::*;
use std::cmp::Reverse;
use std::mem::swap;
use std::iter::FromIterator;
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() <= 1 { return 0; }
        let tgt = arr.len() - 1;

        let mut m = arr.iter().enumerate().fold(HashMap::new(), |mut m, (i,n)| {
            let h = m.entry(*n).or_insert(VecDeque::new());
            h.push_back(i);
            m
        });
        // println!("{:?}", m);

        let mut res = 0;
        let mut v = HashSet::new();
        let (mut q1, mut q2) = (VecDeque::new(), VecDeque::new());
        q2.push_back(0);
        while !q2.is_empty() {
            swap(&mut q1, &mut q2);
            // println!("{:?}", q1);
            while let Some(i) = q1.pop_front() {
                if i == tgt { return res; }
                if v.contains(&i) { continue; }
                v.insert(i);
                let n = arr[i];
                let q = m.get_mut(&n).unwrap();
                q2.append(q);
                if i > 0 { q2.push_back(i - 1); }
                if i < tgt { q2.push_back(i + 1); }
            }
            res += 1;
        }
        unreachable!()
    }
}