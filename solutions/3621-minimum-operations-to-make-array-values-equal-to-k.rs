use std::collections::BTreeSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut set: BTreeSet<_> = nums.into_iter().collect();

        if *set.first().unwrap() < k {
            return -1;
        }
        
        set.split_off(&(k + 1)).len() as i32
    }
}