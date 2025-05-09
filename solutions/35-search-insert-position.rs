use std::cmp::Ordering::{Equal, Greater, Less};
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut h, mut m) = (0, nums.len(), nums.len() / 2);
        while l < h {
            match nums[m].cmp(&target) {
                Equal => return m as i32,
                Greater => h = m,
                Less => l = m + 1,
            }
            m = l + ((h - l) / 2);
        }
        m as i32
    }
}