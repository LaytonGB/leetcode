impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for (i, c1) in text1.chars().enumerate() {
            for (j, c2) in text2.chars().enumerate() {
                dp[i+1][j+1] = if c1 == c2 {
                    dp[i][j] + 1
                } else {
                    dp[i+1][j].max(dp[i][j+1])
                }
            }
        }
        dp[m][n]
    }
}