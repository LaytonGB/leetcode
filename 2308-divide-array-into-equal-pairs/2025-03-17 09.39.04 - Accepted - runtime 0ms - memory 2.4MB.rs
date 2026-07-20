use std::collections::HashMap;
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        nums.into_iter()
            .fold(HashMap::<i32, bool>::new(), |mut counts, n| {
                counts.entry(n)
                    .and_modify(|c| *c = !*c)
                    .or_insert(false);
                counts
            })
            .into_values()
            .all(|c| c)
    }
}