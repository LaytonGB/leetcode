use std::{
    cmp::Ordering as O,
    iter::repeat,
};

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = nums.into_iter()
            .fold([0_usize; 201], |mut counts, n| {
                counts[n as usize + 100] += 1;
                counts
            })
            .into_iter()
            .enumerate()
            .fold(Vec::with_capacity(100), |mut counts, (i, c)| {
                if c != 0 {
                    counts.push((i, c));
                }
                counts
            });

        counts.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));

        counts.into_iter()
            .flat_map(|(i, c)| repeat(i as i32 - 100).take(c))
            .collect()
    }
}