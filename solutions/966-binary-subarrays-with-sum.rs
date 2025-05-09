use std::collections::HashMap;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut counts = HashMap::from([(0, 1)]);
        let mut sum = 0;
        let mut res = 0;

        for x in nums {
            sum += x;

            if let Some(count) = counts.get(&(sum - goal)) {
                res += count;
            }

            counts.entry(sum)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        res
    }
}