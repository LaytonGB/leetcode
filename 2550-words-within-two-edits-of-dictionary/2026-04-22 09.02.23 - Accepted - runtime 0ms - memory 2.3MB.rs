impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries.into_iter()
            .enumerate()
            .filter(|(i, q)| Self::has_diff(&q, &dictionary))
            .map(|(i, q)| q)
            .collect()
    }

    fn has_diff(a: &str, b: &[String]) -> bool {
        b.iter().any(|b| Self::get_diff_count(a, b).is_ok())
    }

    fn get_diff_count(a: &str, b: &str) -> Result<usize, ()> {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut diff_count = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                diff_count += 1;
                if diff_count > 2 {
                    return Err(());
                }
            }
        }
        Ok(diff_count)
    }
}