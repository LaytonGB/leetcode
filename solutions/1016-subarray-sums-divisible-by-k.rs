use std::iter::{FromIterator, once, repeat};


impl Solution {
    pub fn subarrays_div_by_k(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().fold(
            (Vec::from_iter(once(1).chain(repeat(0).take(k as usize - 1))), 0, 0),
            |(mut ps, mut sum, mut res), x| {
                sum += x;
                let mut r = sum.rem_euclid(k) as usize;
                res += ps[r];
                ps[r] += 1;
                (ps, sum, res)
            }
        ).2
    }
}