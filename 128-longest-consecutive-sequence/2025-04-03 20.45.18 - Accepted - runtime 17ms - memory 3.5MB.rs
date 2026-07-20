use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        
        let mut heap = BinaryHeap::from_iter(nums.into_iter());
        
        let mut res = 1;
        let mut curr = 1;
        let mut a = heap.pop().unwrap();
        while let Some(b) = heap.pop() {
            if a == b + 1 {
                curr += 1;
                res = res.max(curr);
            } else if a != b {
                curr = 1;
            }
            
            a = b;
        }

        res
    }
}