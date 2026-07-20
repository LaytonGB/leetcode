use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let (m, n) = (nums1.len(), nums2.len());
        
        let mut h = BinaryHeap::new();
        for i in 0..k.min(m) {
            for j in 0..k.min(n) {
                if h.len() < k {
                    h.push((nums1[i] + nums2[j], nums1[i], nums2[j]));
                    continue;
                }

                if nums1[i] + nums2[j] >= h.peek().unwrap().0 {
                    break;
                }

                h.pop();
                h.push((nums1[i] + nums2[j], nums1[i], nums2[j]));
            }
        }

        (0..k).map(|_| h.pop().unwrap())
            .map(|(_, a, b)| vec![a, b])
            .collect()
    }
}