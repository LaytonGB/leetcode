use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(a.len());
        let mut a_mask = 0_u64;
        let mut b_mask = 0_u64;

        for (a, b) in a.into_iter().zip(b.into_iter()) {
            a_mask |= 1_u64 << a;
            b_mask |= 1_u64 << b;

            res.push((a_mask & b_mask).count_ones() as i32);
        }

        res
    }
}