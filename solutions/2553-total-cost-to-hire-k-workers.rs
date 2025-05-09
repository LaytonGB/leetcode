use std::collections::*;
use std::cmp::Reverse;
impl Solution {
    pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut candidates = candidates as usize;
        let mut res = 0_i64;
        let (mut h1, mut h2) = (BinaryHeap::new(), BinaryHeap::new());
        let (mut low_ptr, mut high_ptr) = (0, costs.len() - 1);
        for _ in 0..k {
            while h1.len() < candidates && low_ptr <= high_ptr {
                h1.push(Reverse(costs[low_ptr]));
                low_ptr += 1;
            }
            while h2.len() < candidates && high_ptr >= low_ptr {
                h2.push(Reverse(costs[high_ptr]));
                high_ptr -= 1;
            }

            let cand_a_cost = if !h1.is_empty() {
                let &Reverse(x) = h1.peek().unwrap();
                x
            } else {
                i32::MAX
            };
            let cand_b_cost = if !h2.is_empty() {
                let &Reverse(x) = h2.peek().unwrap();
                x
            } else {
                i32::MAX
            };

            if cand_a_cost <= cand_b_cost {
                res += cand_a_cost as i64;
                h1.pop();
            } else {
                res += cand_b_cost as i64;
                h2.pop();
            }

            // match (h1.pop(), h2.pop()) {
            //     (Some(Reverse(a)), Some(Reverse(b))) => {
            //         if a <= b {
            //             res += a as i64;
            //             h2.push(Reverse(b));
            //         } else {
            //             res += b as i64;
            //             h1.push(Reverse(a));
            //         }
            //     }
            //     (Some(Reverse(a)), None) | (None, Some(Reverse(a))) => {
            //         res += a as i64;
            //     }
            //     _ => { unreachable!(); }
            // }
        }
        res
    }
}