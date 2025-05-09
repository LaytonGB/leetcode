use std::cmp::Ordering as O;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        if nums1.last().unwrap() < nums2.first().unwrap()
        || nums1.first().unwrap() > nums2.last().unwrap() {
                return -1;
        }
        
        let (n1, n2) = (nums1.len(), nums2.len());
        let (mut i, mut j) = (0, 0);
        while i < n1 && j < n2 {
            match (nums1[i].cmp(&nums2[j])) {
                O::Equal => return nums1[i],
                O::Less => i += 1,
                O::Greater => j += 1,
            }
        }

        -1
    }
}