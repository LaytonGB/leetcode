impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        // answer from https://leetcode.com/problems/perfect-squares/solutions/4694883/beats-99-users-c-java-python-javascript-explained
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            let mut j = 1;
            while j * j <= i {
                dp[i] = dp[i].min(dp[i - j.pow(2)] + 1);
                j += 1;
            }
        }
        dp[n]
    }
}