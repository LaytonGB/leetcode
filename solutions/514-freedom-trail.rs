// answer from: https://leetcode.com/problems/freedom-trail/solutions/98902/concise-java-dp-solution
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let m = key.len();

        let ring = ring.into_bytes();
        let key = key.into_bytes();
        
        let mut dp = vec![vec![0; n]; m + 1];

        for i in (0..m).rev() {
            for j in 0..n {
                dp[i][j] = usize::MAX;
                for k in 0..n {
                    if ring[k] == key[i] {
                        let diff = j.abs_diff(k);
                        let step = diff.min(n - diff);
                        dp[i][j] = dp[i][j].min(step + dp[i + 1][k]);
                    }
                }
            }
        }

        (dp[0][0] + m) as i32
    }
}