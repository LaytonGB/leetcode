impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words.into_iter()
            .find(|s| 
                s.chars().zip(s.chars().rev()).take(s.len() / 2).all(|(a, b)| a == b)
            )
            .unwrap_or_default()
    }
}