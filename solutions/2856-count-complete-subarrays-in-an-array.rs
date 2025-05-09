use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let target = nums.iter().collect::<HashSet<_>>().len();
        let mut count = 0; // count of unique values
        let mut seen = HashMap::<i32, usize>::with_capacity(target);
        let mut l = 0;
        let mut res = 0;
        
        for r in 0..n {
            let mut entry = seen.entry(nums[r]).or_default();
            *entry += 1;
            if *entry == 1 {
                count += 1;
            }

            while count == target {
                res += n - r;
                let mut entry = seen.get_mut(&nums[l]).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    count -= 1;
                }
                l += 1;
            }
        }

        res as i32
    }
}