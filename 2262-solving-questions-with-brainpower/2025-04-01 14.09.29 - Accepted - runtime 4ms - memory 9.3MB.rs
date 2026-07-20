impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();

        let mut dp = vec![0; n];
        (0..n).rev().for_each(|i| {
            if i + questions[i][1] as usize + 1 >= n {
                dp[i] = questions[i][0] as i64;
            } else {
                dp[i] = dp[i + questions[i][1] as usize + 1] + questions[i][0] as i64;
            }
            if i < n - 1 {
                dp[i] = dp[i].max(dp[i + 1]);
            }
        });

        // println!("{:?}", dp);

        dp[0]
    }
}