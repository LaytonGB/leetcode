use std::collections::BTreeSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        
        let mut set = BTreeSet::from_iter(nums.into_iter());
        
        let mut res = 1;
        let mut curr = 1;
        let mut a = i32::MIN;
        for b in set {
            if a + 1 == b {
                curr += 1;
                res = res.max(curr);
            } else {
                curr = 1;
            }
            
            a = b;
        }

        res
    }
}