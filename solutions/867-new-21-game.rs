impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 || n >= k + max_pts { return 1.; }
        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![0.; n + 1];
        dp[0] = 1.;
        let mut pts = 1.;
        for i in 1 ..= n {
            dp[i] = pts / max_pts as f64;
            if i < k { pts += dp[i]; }
            if let Some(j) = i.checked_sub(max_pts as usize) { pts -= dp[j]; }
        }
        dp[k..].iter().sum()
    }
}