use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct OrdFloat(f64);
impl Eq for OrdFloat {}
impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut h = BinaryHeap::with_capacity(arr.len().pow(2));
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                h.push((OrdFloat(arr[i] as f64 / arr[j] as f64), [arr[i], arr[j]]));
            }
        }
        
        let v = h.into_sorted_vec();

        v[k as usize - 1].1.to_vec()
    }
}