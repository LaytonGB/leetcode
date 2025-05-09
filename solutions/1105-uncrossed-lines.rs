use std::collections::*;
impl Solution {
    pub fn max_uncrossed_lines(n1: Vec<i32>, n2: Vec<i32>) -> i32 {
        let (m, n) = (n1.len(), n2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if n1[i-1] == n2[j-1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
                }
            }
        }
        dp[m][n]
    }
}