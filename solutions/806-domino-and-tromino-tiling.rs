const MOD: usize = 10_usize.pow(9) + 7;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let n = n as usize;

        let mut dp: Vec<usize> = vec![0, 1, 2, 5];
        for i in 4..=n {
            dp.push((dp[i - 1] * 2 + dp[i - 3]) % MOD);
        }

        dp[n] as i32
    }
}