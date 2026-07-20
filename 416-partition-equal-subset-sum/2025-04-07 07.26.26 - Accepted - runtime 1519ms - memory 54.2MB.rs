use std::collections::HashSet;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        Self::r(&nums, &mut HashSet::new(), 0, 0, 0)
    }

    fn r(
        nums: &[i32],
        memo: &mut HashSet<(usize, i32, i32)>,
        i: usize,
        a: i32,
        b: i32
    ) -> bool {
        if i == nums.len() {
            a == b
        } else if memo.contains(&(i, a, b)) {
            false
        } else {
            if Self::r(nums, memo, i + 1, a + nums[i], b)
            || Self::r(nums, memo, i + 1, a, b + nums[i]) {
                true
            } else {
                memo.insert((i, a, b));
                false
            }
        }
    }
}