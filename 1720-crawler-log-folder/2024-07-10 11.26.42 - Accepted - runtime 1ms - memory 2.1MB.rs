impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.into_iter()
            .fold(0_i32, |dist, l| match l.as_str() {
                "./" => dist,
                "../" => dist - (dist > 0) as i32,
                _ => dist + 1,
            })
    }
}