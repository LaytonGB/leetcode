use std::{
    cmp::{Ordering as O, Reverse},
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

        counts.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
            O::Equal => Reverse(a.0).cmp(&Reverse(b.0)),
            o => o,
        });

        counts.into_iter()
            .flat_map(|(i, c)| repeat(i as i32 - 100).take(c))
            .collect()
    }
}