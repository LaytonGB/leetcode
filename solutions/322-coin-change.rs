impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {        
        let mut dp = vec![-1; amount as usize + 1];
        dp[0] = 0;

        for i in 0..amount as usize {
            if dp[i] == -1 {
                continue;
            }

            for &c in coins.iter() {
                let idx = i + c as usize;
                if idx > amount as usize {
                    continue;
                }

                if dp[idx] == -1 {
                    dp[idx] = dp[i] + 1;
                } else {
                    dp[idx] = dp[idx].min(dp[i] + 1);
                }
            }
        }

        dp[amount as usize]
    }
}