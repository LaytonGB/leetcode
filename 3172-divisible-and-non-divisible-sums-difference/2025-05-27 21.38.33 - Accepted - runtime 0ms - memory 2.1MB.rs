impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let n_sum = n * (n + 1) / 2;
        let m_sum = (1..=n / m).map(|i| i * m).sum::<i32>();
        n_sum - m_sum - m_sum
    }
}