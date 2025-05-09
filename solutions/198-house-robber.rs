impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return *nums.iter().max().unwrap();
        }
        
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        dp[1] = nums[1];
        dp[2] = nums[2] + dp[0];

        for i in 3..n {
            dp[i] = dp[i-1].max(dp[i-2] + nums[i]).max(nums[i] + dp[i - 3]);
        }

        dp[n - 1].max(dp[n - 2])
    }
}