impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let n = weights.len();
        let k = k as usize;
        if k == 1 || k == n {
            return 0;
        }

        let mut pair_weights: Vec<i32> = weights.windows(2).map(|w| w[0] + w[1]).collect();
        pair_weights.sort_unstable();

        (0..k - 1).map(|i| pair_weights[n - i - 2] as i64 - pair_weights[i] as i64)
            .sum::<i64>()
    }
}