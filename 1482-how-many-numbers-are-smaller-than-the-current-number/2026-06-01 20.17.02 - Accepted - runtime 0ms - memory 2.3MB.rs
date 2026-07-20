use std::collections::HashMap;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut indices: Vec<usize> = (0..nums.len()).collect();
        indices.sort_unstable_by_key(|i| nums[*i]);
        // println!("{:?}", indices);

        let lookup = indices.into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut res, (count, val_idx)| {
                res.entry(nums[val_idx]).or_insert(count as i32);
                res
            });
        // println!("{:?}", lookup);

        nums.into_iter().map(|x| *lookup.get(&x).unwrap()).collect()
    }
}