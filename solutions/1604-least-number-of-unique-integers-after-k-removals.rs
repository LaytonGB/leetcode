use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        if k == arr.len() as i32 {
            return 0;
        }
        
        let count = arr.iter().fold(HashMap::with_capacity(arr.len()), |mut m, x| {
            m.entry(*x)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            m
        });

        if k == 0 {
            return count.len() as i32;
        }

        let mut heap = BinaryHeap::from_iter(count.iter().map(|(k, v)| (Reverse(v), k)));
        let mut removed = 0;
        while let Some((Reverse(count), _)) = heap.pop() {
            if k >= *count {
                removed += 1;
                k -= count;
            } else {
                break;
            }
        }

        count.len() as i32 - removed
    }
}