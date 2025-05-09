use std::collections::{HashSet, hash_map::RandomState as S};
use std::iter::FromIterator;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1 = HashSet::<i32, S>::from_iter(nums1.into_iter());
        HashSet::<i32, S>::from_iter(
            nums2.into_iter().filter(|x| s1.contains(x))
        )
            .into_iter()
            .collect()
    }
}