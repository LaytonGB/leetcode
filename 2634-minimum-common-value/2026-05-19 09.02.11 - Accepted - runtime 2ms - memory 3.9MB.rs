use std::cmp::Ordering as O;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            let (a, b) = (nums1[i], nums2[j]);
            match a.cmp(&b) {
                O::Equal => return a,
                O::Less => while i < m && nums1[i] == a {
                    i += 1;
                },
                O::Greater => while j < n && nums2[j] == b {
                    j += 1;
                },
            }
        }

        -1
    }
}