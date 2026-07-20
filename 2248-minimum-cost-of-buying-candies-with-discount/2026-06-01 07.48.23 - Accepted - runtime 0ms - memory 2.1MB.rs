impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        let n = cost.len();
        (0..n).rev()
            .filter(|i| (n - i) % 3 != 0)
            .map(|i| cost[i])
            .fold(0, |res, c| res + c)
    }
}