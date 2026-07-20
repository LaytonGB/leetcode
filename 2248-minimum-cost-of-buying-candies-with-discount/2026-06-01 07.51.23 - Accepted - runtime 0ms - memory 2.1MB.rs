impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();
        cost.into_iter()
            .rev()
            .enumerate()
            .filter(|(i, _)| (i + 1) % 3 != 0)
            .fold(0, |res, (_, c)| res + c)
    }
}