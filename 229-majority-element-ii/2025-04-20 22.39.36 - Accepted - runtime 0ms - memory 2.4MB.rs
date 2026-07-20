use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let lim = nums.len() / 3;
        let mut counts = HashMap::new();
        let mut res = HashSet::new();
        for x in nums.into_iter() {
            if *counts.entry(x)
                .and_modify(|e| *e += 1)
                .or_insert(1)
                > lim
            {
                res.insert(x);
            }
        }
        res.into_iter().collect()
    }
}