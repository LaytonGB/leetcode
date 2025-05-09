impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; cost.len() + 3];
        dp[0] = 0;
        dp[1] = 0;
        
        for i in 0..cost.len() {
            dp[i + 1] = dp[i + 1].min(dp[i] + cost[i]);
            dp[i + 2] = dp[i + 2].min(dp[i] + cost[i]);
        }

        dp[cost.len()]
    }
}