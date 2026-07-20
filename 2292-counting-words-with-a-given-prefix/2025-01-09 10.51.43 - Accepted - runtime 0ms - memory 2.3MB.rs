impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.into_iter().fold(0, |sum, w| sum + w.starts_with(&pref) as i32)
    }
}