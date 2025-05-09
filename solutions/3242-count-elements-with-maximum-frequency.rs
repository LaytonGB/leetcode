use std::collections::HashMap;
impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let freq = nums.iter().fold(HashMap::new(), |mut m, n| {
            m.entry(n)
                .and_modify(|x| *x += 1)
                .or_insert(1);
                m
        });
        let max = freq.values().max().unwrap();
        freq.values()
            .filter(|x| *x == max)
            .sum()
    }
}