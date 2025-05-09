// https://leetcode.com/problems/the-number-of-beautiful-subsets/solutions/3314361/python-house-robber-o-n

use std::collections::HashMap;
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = vec![HashMap::<i32, i32>::new(); k as usize];
        for &n in nums.iter() {
            *count[(n % k) as usize].entry(n)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let mut res = 1;
        for i in 0..k as usize {
            let (mut prev, mut dp0, mut dp1) = (0, 1, 0);

            let mut keys = count[i].keys().collect::<Vec<_>>();
            keys.sort_unstable();
            for &c in keys.iter() {
                let v = 2_i32.pow(*count[i].get(c).unwrap() as u32) - 1;

                if prev + k == *c {
                    (dp0, dp1) = (dp0 + dp1, dp0 * v);
                } else {
                    (dp0, dp1) = (dp0 + dp1, (dp0 + dp1) * v);
                }

                prev = *c;
            }

            res *= dp0 + dp1;
        }

        res - 1
    }
}