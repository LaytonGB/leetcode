use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq_map = HashMap::new();
        for c in s.chars() {
            freq_map.entry(c)
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }

        let mut freq_heap = BinaryHeap::new();
        for (c, n) in freq_map {
            freq_heap.push((n, c));
        }

        let mut res = Vec::with_capacity(s.len());
        while let Some((n, c)) = freq_heap.pop() {
            for _ in 0..n {
                res.push(c);
            }
        }
        
        res.into_iter().collect()
    }
}