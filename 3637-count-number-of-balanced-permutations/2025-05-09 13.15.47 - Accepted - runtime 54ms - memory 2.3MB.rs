const MOD: usize = 10_usize.pow(9) + 7;

impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let n = num.len();
        let nums: Vec<usize> = num.bytes().map(|b| (b - b'0') as usize).collect();
        let total: usize = nums.iter().sum();

        if total & 1 == 1 {
            return 0;
        }

        let mut factorials = vec![1; n + 1];
        (1..=n).for_each(|i| factorials[i] = (factorials[i - 1] * i) % MOD);

        let mut inverses = vec![1; n + 1];
        (2..=n).for_each(|i| inverses[i] = MOD - (MOD / i * inverses[MOD % i]) % MOD);

        let mut inverse_factorials = vec![1; n + 1];
        (1..=n).for_each(|i| inverse_factorials[i] = (inverse_factorials[i - 1] * inverses[i]) % MOD);

        let half_sum = total / 2;
        let half_len = n / 2;

        let mut dp = vec![vec![0; half_len + 1]; half_sum + 1];
        dp[0][0] = 1;

        let mut num_counts = [0; 10];
        for &x in nums.iter() {
            num_counts[x] += 1;
            for i in (x..=half_sum).rev() {
                for j in (1..=half_len).rev() {
                    dp[i][j] = (dp[i][j] + dp[i - x][j - 1]) % MOD;
                }
            }
        }

        let mut res = dp[half_sum][half_len];
        res = (res * factorials[half_len]) % MOD * factorials[n - half_len] % MOD;

        for c in num_counts.into_iter() {
            res = (res * inverse_factorials[c]) % MOD;
        }

        res as i32
    }
}