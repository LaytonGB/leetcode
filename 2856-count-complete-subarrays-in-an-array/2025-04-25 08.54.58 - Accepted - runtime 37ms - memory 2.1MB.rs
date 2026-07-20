use std::collections::HashSet;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let unique_count = nums.iter().copied().collect::<HashSet<i32>>().len();
        let mut res = 0;
        for i in 0..=n - unique_count {
            let mut unique = HashSet::with_capacity(unique_count);
            for j in i..n {
                unique.insert(nums[j]);
                if unique.len() == unique_count {
                    res += n - j;
                    break;
                }
            }
        }
        res as i32
    }
}