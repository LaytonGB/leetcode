// from https://leetcode.com/problems/minimum-cost-to-hire-k-workers/solutions/5141342/fastest-100-easy-to-understand-clean-concise

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct OrdFloat(pub f64);
impl Eq for OrdFloat {}
impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut quality_heap = BinaryHeap::<i32>::with_capacity(k as usize);
        let mut max_ratio = 0.0_f64;
        let mut total_quality = 0;
        let mut ratios = quality.iter().zip(wage.iter())
            .map(|(q, w)| (OrdFloat((*w) as f64 / (*q) as f64), q))
            .collect::<Vec<_>>();
        ratios.sort_unstable();

        for i in 0..k as usize {
            let (r, q) = ratios[i];
            max_ratio = max_ratio.max(r.0);
            total_quality += q;
            quality_heap.push(*q);
        }

        let mut res = max_ratio * total_quality as f64;

        for i in k as usize..quality.len() {
            let (r, q) = ratios[i];
            max_ratio = max_ratio.max(r.0);
            total_quality += q - quality_heap.pop().unwrap();
            quality_heap.push(*q);
            res = res.min(max_ratio * total_quality as f64);
        }

        res
    }
}