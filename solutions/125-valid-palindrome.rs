impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let fwd = s.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();
        fwd.eq(&fwd.chars().rev().collect::<String>())
    }
}