use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        let counts = nums.iter()
            .fold(HashMap::<i32, i32>::with_capacity(201), |mut m, n| {
                *m.entry(*n).or_default() += 1;
                m
            });
        
        nums.sort_unstable_by(|a, b| counts.get(a).unwrap().cmp(counts.get(b).unwrap()).then(b.cmp(a)));

        nums
    }
}