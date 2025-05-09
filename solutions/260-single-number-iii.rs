use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut singles = HashSet::with_capacity(nums.len() / 2);
        for n in nums.into_iter() {
            if !singles.insert(n) {
                singles.remove(&n);
            }
        }
        singles.into_iter().collect()
    }
}