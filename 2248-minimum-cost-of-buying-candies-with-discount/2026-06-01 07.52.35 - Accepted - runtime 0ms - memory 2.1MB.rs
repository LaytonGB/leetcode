impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable_by(|a, b| b.cmp(a));
        cost.into_iter()
            .enumerate()
            .filter(|(i, _)| i % 3 != 2)
            .fold(0, |res, (_, c)| res + c)
    }
}