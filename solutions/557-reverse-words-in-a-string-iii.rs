impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|mut w| w.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}