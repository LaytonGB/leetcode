use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let n = arr1.len();
        if n == 1 {
            return arr1;
        }
        
        let mut pos = [None; 1001];
        for (i, x) in arr2.into_iter().enumerate() {
            pos[x as usize] = Some(i);
        }
        let mut heap = BinaryHeap::with_capacity(n);
        for x in arr1 {
            heap.push((Reverse(pos[x as usize].unwrap_or(1001 + x as usize)), x));
        }
        let mut res = Vec::with_capacity(n);
        while let Some((_, x)) = heap.pop() {
            res.push(x);
        }
        res
    }
}