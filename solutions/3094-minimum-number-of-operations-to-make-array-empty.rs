use std::collections::*;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let counts = nums.into_iter().fold(HashMap::new(), |mut m, x| {
            m.entry(x)
                .and_modify(|x| *x += 1)
                .or_insert(1);
            m
        });

        let mut res = 0;
        for mut x in counts.into_values() {
            while x > 0 && x % 3 != 0 {
                x -= 2;
                res += 1;
            }
            if x % 3 == 0 {
                res += x / 3;
            } else {
                return -1;
            }
        }
        res
    }
}