use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut prefix = HashMap::<i32, i64>::new();
        let mut suffix = HashMap::<i32, i64>::new();

        // build suffix
        for &x in nums.iter() {
            *suffix.entry(x).or_insert(0) += 1;
        }

        // shift values into prefix and accumulate result
        let mut res = 0_i64;
        for &x in nums.iter() {
            *suffix.entry(x).or_insert(0) -= 1;
            res += (*prefix.get(&(x*2)).unwrap_or(&0) * *suffix.get(&(x*2)).unwrap_or(&0)) as i64;
            *prefix.entry(x).or_insert(0) += 1;
        }

        (res % MOD) as i32
    }
}