impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence.split(' ')
            .position(|w| w.starts_with(&search_word))
            .map(|i| (i + 1) as i32)
            .unwrap_or(-1)
    }
}