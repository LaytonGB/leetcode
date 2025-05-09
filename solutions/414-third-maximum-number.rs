use std::collections::HashSet;

impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        let mut n = nums.iter()
        .fold(HashSet::<i32>::new(), |mut s, n| {
            s.insert(*n);
            s
        })
        .iter().map(|n| *n)
        .collect::<Vec<i32>>();
        if n.len() < 3 {
            return *n.iter().max().unwrap();
        }
        n.sort();
        n[n.len() - 3]
    }
}