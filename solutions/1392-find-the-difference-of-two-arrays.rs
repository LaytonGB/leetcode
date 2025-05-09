use std::collections::BTreeSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let make_bset = |mut set: BTreeSet<i32>, n: &i32| {
            set.insert(*n);
            set
        };
        let a = nums1.iter()
            .fold(BTreeSet::new(), make_bset);
        let b = nums2.iter()
            .fold(BTreeSet::new(), make_bset);
        vec![a.difference(&b).map(|n| *n).collect(), b.difference(&a).map(|n| *n).collect()]
    }
}