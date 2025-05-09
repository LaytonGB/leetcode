use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::<i32, i32>::with_capacity(nums.len());
        for (i, &n) in nums.iter().enumerate() {
            let e = map.entry(n).or_insert(-1);
            if *e != -1 && i as i32 - *e <= k {
                return true;
            } else {
                *e = i as i32;
            }
        }
        false
    }
}