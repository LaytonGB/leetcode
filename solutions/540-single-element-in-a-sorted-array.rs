use std::collections::HashSet;
use std::cmp::Ordering::*;
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut h) = (0, nums.len() - 1);
        let mut m = l;
        while l < h {
            m = (l + h) / 2;
            if nums[m] == nums[m ^ 1] { // m ^ 1 -> m XOR 1 -> compare even nums to next element, odd nums to previous element
                l = m + 1;
            } else {
                h = m;
            }
        }
        nums[l]
    }
}