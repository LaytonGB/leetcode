use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut sets = HashMap::from([(-1, HashSet::new())]);
        for x in nums {
            sets.insert(
                x,
                sets.keys().filter(|d| x % **d == 0)
                    .map(|d| sets.get(d).unwrap())
                    .max_by_key(|s| s.len())
                    .unwrap()
                    .union(&HashSet::from([x]))
                    .copied()
                    .collect()
            );
        }

        sets.values()
            .max_by_key(|s| s.len())
            .unwrap()
            .iter()
            .copied()
            .collect()
    }
}