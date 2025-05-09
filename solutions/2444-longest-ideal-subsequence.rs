impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = [0_i32; 150];
        let k = k as usize;
        for c in s.bytes() {
            let c = c as usize;
            dp[c] = dp[c - k..=c + k].iter().max().unwrap() + 1;
        }
        *dp.iter().max().unwrap()
    }
}