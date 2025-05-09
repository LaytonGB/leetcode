impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut counts = roads.into_iter()
            .fold(vec![0_i64; n as usize], |mut v, r| {
                v[r[0] as usize] += 1;
                v[r[1] as usize] += 1;
                v
            });
        counts.sort_unstable();

        counts.into_iter()
            .enumerate()
            .fold(0_i64, |res, (i, c)| {
                res + c * (i + 1) as i64
            })
    }
}