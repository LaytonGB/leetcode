impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by_key(|c| -c);
        citations.into_iter()
            .enumerate()
            .fold(0_i32, |res, (i, c)| {
                if c >= i as i32 + 1 {
                    res.max(c.min(i as i32 + 1))
                } else {
                    res
                }
            })
    }
}