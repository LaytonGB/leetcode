use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if nums1.len() == 0 || nums2.len() == 0 || k == 0 {
            return vec![];
        }
        
        let k = k as usize;
        
        let mut q: BinaryHeap<(Reverse<i32>, i32, i32, usize)> = BinaryHeap::new();
        for &u in nums1.iter().take(k) {
            q.push((Reverse(u + nums2[0]), u, nums2[0], 0));
        }

        let mut res = Vec::with_capacity(k);
        while let Some((_, u, v, i)) = q.pop() {
            res.push(vec![u, v]);
            if res.len() == k {
                break;
            }
            if i < nums2.len() - 1 {
                q.push((Reverse(u + nums2[i + 1]), u, nums2[i + 1], i + 1));
            }
        }

        res
    }
}