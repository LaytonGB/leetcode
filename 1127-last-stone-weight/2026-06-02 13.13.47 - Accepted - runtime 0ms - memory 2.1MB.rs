use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut h = BinaryHeap::from(stones);
        while h.len() > 1 {
            let a = h.pop().unwrap();
            let b = h.pop().unwrap();
            let rem = a.abs_diff(b);
            if rem != 0 {
                h.push(rem as i32);
            }
        }
        h.pop().unwrap_or(0)
    }
}