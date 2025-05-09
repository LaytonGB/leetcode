use std::collections::*;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = BinaryHeap::from(stones);
        while stones.len() > 1 {
            let (y, x) = (stones.pop().unwrap(), stones.pop().unwrap());
            if y != x {
                stones.push(y - x);
            }
        }
        if stones.is_empty() { 0 } else { *stones.peek().unwrap() }
    }
}