impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (coins.len(), coins[0].len());
        let mut dp = vec![vec![[0; 3]; n]; m];
        for i in 0..m {
            for j in 0..n {
                dp[i][j] = Self::get_dp_for_pos(&coins, &dp, i, j);
            }
        }

        dp[m-1][n-1].into_iter().max().unwrap()
    }

    fn get_dp_for_pos(coins: &Vec<Vec<i32>>, dp: &Vec<Vec<[i32; 3]>>, i: usize, j: usize) -> [i32; 3] {
        let c = coins[i][j];
        let is_robber = c < 0;
        
        match (i > 0, j > 0) {
            (false, false) => [
                c,
                c.max(0),
                c.max(0),
            ],
            (true, false) => [
                (dp[i-1][j][0] + c),
                (dp[i-1][j][0]).max(dp[i-1][j][1] + c),
                (dp[i-1][j][1]).max(dp[i-1][j][2] + c),
            ],
            (false, true) => [
                (dp[i][j-1][0] + c),
                (dp[i][j-1][0]).max(dp[i][j-1][1] + c),
                (dp[i][j-1][1]).max(dp[i][j-1][2] + c),
            ],
            (true, true) => [
                (dp[i-1][j][0]).max(dp[i][j-1][0]) + c,
                (dp[i-1][j][0]).max(dp[i-1][j][1] + c).max(dp[i][j-1][0]).max(dp[i][j-1][1] + c),
                (dp[i-1][j][1]).max(dp[i-1][j][2] + c).max(dp[i][j-1][1]).max(dp[i][j-1][2] + c),
            ]
        }
    }
}