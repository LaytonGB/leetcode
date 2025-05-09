use std::collections::*;
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut dp = vec![0];
        let mut res = 0;
        for s in &arr {
            // filter invalid strings
            let mut a = 0; // bit string of present characters
            let mut dup = 0; // temp bit string to check for duplicates
            for b in s.bytes() {
                let b = (b - b'a'); 
                dup |= a & (1 << b); // convert char to bitflag, compare to a, apply to dup
                a |= 1 << b; // apply bitflag to a
            }
            if dup > 0 {
                continue;
            }

            // add to dp
            for i in (0..dp.len()).rev() {
                if dp[i] & a > 0 { // continue if any duplicate char
                    continue;
                }
                let new_dp: i32 = dp[i] | a;
                dp.push(new_dp);
                res = res.max(new_dp.count_ones());
            }
        }
        res as i32
    }
}