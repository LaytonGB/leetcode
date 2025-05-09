impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let idx = word.chars().position(|c| c == ch);

        match idx {
            Some(i) => word[..=i].chars().rev().chain(word[i + 1..].chars()).collect(),
            None => word
        }
    }
}