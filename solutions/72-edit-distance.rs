use std::collections::HashMap;

impl Solution {
    pub fn min_distance(w1: String, w2: String) -> i32 {
        let (m, n) = (w1.len(), w2.len());

        let mut dp = vec![vec![0; w2.len() + 1]; w1.len() + 1];

        // worst state is to remove all then push all.
        // these initial states assume removing all.
        for i in 1..m + 1 {
            dp[i][0] = i as i32;
        }
        for j in 1..n + 1 {
            dp[0][j] = j as i32;
        }

        // try to find a cheap path top-left to bottom-right
        for (i, c) in w1.chars().enumerate() {
            for (j, d) in w2.chars().enumerate() {
                dp[i + 1][j + 1] = (dp[i][j] + (c != d) as i32)
                    .min(dp[i][j + 1] + 1)
                    .min(dp[i + 1][j] + 1);
            }
        }

        // bottom right value must be cheapest
        dp[m][n]
    }
}