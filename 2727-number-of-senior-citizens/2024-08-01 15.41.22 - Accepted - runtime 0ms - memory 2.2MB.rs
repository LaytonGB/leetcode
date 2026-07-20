impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details.into_iter()
            .filter(|d| (&d[11..=12]).parse::<u8>().is_ok_and(|age| age > 60))
            .count() as i32
    }
}