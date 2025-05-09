use std::cmp::Ordering as O;
impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        if nums1[0] > *nums2.last().unwrap() || *nums1.last().unwrap() < nums2[0] {
            return Vec::with_capacity(0);
        }

        let (mut i, mut j) = (0, 0);
        let mut res = Vec::with_capacity(1000);
        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                O::Less => i += 1,
                O::Equal => {
                    res.push(nums1[i]);
                    i += 1;
                    j += 1;
                }
                O::Greater => j += 1,
            }
        }

        res
    }
}