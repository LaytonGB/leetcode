use std::collections::*;
use std::cmp::Reverse;
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums: Vec<(i64, i64)> = nums2.into_iter().zip(nums1.into_iter()).map(|(b,a)| (b as i64, a as i64)).collect();
        nums.sort_unstable_by(|a,b| b.partial_cmp(a).unwrap());
        let mut pq = BinaryHeap::with_capacity(k + 1);
        let (mut res, mut sum) = (0_i64, 0_i64);
        for (b,a) in nums.into_iter() {
            pq.push(Reverse(a));
            sum += a;
            if pq.len() > k {
                let Reverse(a) = pq.pop().unwrap();
                sum -= a;
            }
            if pq.len() == k {
                res = res.max(sum * b);
            }
        }
        res
    }
}