impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let sum = nums.iter().sum::<i32>();
        if sum & 1 == 1 {
            return false;
        }

        let tgt = sum as usize / 2;
        
        let mut dp = vec![false; tgt + 1];
        dp[0] = true;

        for i in 1..n {
            for j in (0..=tgt).rev() {
                if j >= nums[i - 1] as usize {
                    dp[j] = dp[j - nums[i - 1] as usize] || dp[j];
                }
            }
        }

        dp[tgt]
    }
}